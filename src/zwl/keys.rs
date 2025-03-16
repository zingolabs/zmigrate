use std::io::{self, ErrorKind};

use anyhow::Result;
use byteorder::{LittleEndian, ReadBytesExt};
use zcash_encoding::Vector;
use zingolib::wallet::utils::read_string;

use crate::{ParseWithParam, Parser};

use super::{
    lightclient::LightClientConfig, walletokey::WalletOKey, wallettkey::WalletTKey,
    walletzkey::WalletZKey,
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct Keys<P> {
    config: LightClientConfig<P>,
    // Is the wallet encrypted? If it is, then when writing to disk, the seed is always encrypted
    // and the individual spending keys are not written
    pub encrypted: bool,

    pub enc_seed: [u8; 48], // If locked, this contains the encrypted seed
    pub nonce: Vec<u8>,     // Nonce used to encrypt the wallet.

    pub seed: [u8; 32], // Seed phrase for this wallet. If wallet is locked, this is 0

    // List of keys, actually in this wallet. This is a combination of HD keys derived from the seed,
    // viewing keys and imported spending keys.
    pub zkeys: Vec<WalletZKey>,

    // Transparent keys. If the wallet is locked, then the secret keys will be encrypted,
    // but the addresses will be present. This Vec contains both wallet and imported tkeys
    pub tkeys: Vec<WalletTKey>,

    // Unified address (Orchard) keys actually in this wallet.
    // If wallet is locked, only viewing keys are present.
    pub okeys: Vec<WalletOKey>,
}

impl<P: zcash_protocol::consensus::Parameters> Keys<P> {
    pub fn serialized_version() -> u64 {
        22
    }

    pub fn read<R: ReadBytesExt>(mut reader: R, config: LightClientConfig<P>) -> io::Result<Self> {
        let version = reader.read_u64::<LittleEndian>()?;

        if version > Self::serialized_version() {
            let e = format!(
                "Don't know how to read `Keys` struct version {}. Do you have the latest version?",
                version
            );
            return Err(io::Error::new(io::ErrorKind::InvalidData, e));
        }

        // Read if wallet is encrypted
        let encrypted = reader.read_u8()? > 0;

        // Read "possible" encypted seed
        let mut enc_seed = [0u8; 48];
        reader.read_exact(&mut enc_seed)?;

        // Read nounce used for encyption
        let nonce = Vector::read(&mut reader, |r| r.read_u8())?;

        // Read "possible" clear seed
        let mut seed_bytes = [0u8; 32];
        reader.read_exact(&mut seed_bytes)?;

        let okeys = if version <= 21 {
            vec![]
        } else {
            Vector::read(&mut reader, |r| WalletOKey::read(r))?
        };

        // TODO: read old versions of wallet file
        let zkeys = Vector::read(&mut reader, |r| WalletZKey::read(r))?;

        // read wallet tkeys

        let tkeys = if version <= 20 {
            let tkeys = Vector::read(&mut reader, |r| {
                let mut tpk_bytes = [0u8; 32];
                r.read_exact(&mut tpk_bytes)?;
                secp256k1::SecretKey::from_slice(&tpk_bytes)
                    .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
            })?;

            let taddresses = Vector::read(&mut reader, |r| read_string(r))?;

            tkeys
                .iter()
                .zip(taddresses.iter())
                .enumerate()
                .map(|(i, (sk, taddr))| WalletTKey::from_raw(sk, taddr, i as u32))
                .collect::<Vec<_>>()
        } else {
            // Read the TKeys
            Vector::read(&mut reader, |r| WalletTKey::read(r))?
        };

        Ok(Self {
            config: config.clone(),
            encrypted,
            enc_seed,
            nonce,
            seed: seed_bytes,
            zkeys,
            tkeys,
            okeys,
        })
    }

    #[allow(dead_code, deprecated)]
    pub fn read_old<R: ReadBytesExt>(
        version: u64,
        mut reader: R,
        config: LightClientConfig<P>,
    ) -> io::Result<Self> {
        let encrypted = if version >= 4 {
            reader.read_u8()? > 0
        } else {
            false
        };

        let mut enc_seed = [0u8; 48];
        if version >= 4 {
            reader.read_exact(&mut enc_seed)?;
        }

        let nonce = if version >= 4 {
            Vector::read(&mut reader, |r| r.read_u8())?
        } else {
            vec![]
        };

        // Seed
        let mut seed_bytes = [0u8; 32];
        reader.read_exact(&mut seed_bytes)?;

        let zkeys = if version <= 6 {
            // Up until version 6, the wallet keys were written out individually
            // Read the spending keys
            let extsks = Vector::read(&mut reader, |r| {
                sapling::zip32::ExtendedSpendingKey::read(r)
            })?;

            let extfvks = if version >= 4 {
                // Read the viewing keys
                Vector::read(&mut reader, |r| {
                    sapling::zip32::ExtendedFullViewingKey::read(r)
                })?
            } else {
                // Calculate the viewing keys
                extsks
                    .iter()
                    .map(|sk| sk.to_extended_full_viewing_key()) // TODO: This is deprecated
                    .collect::<Vec<sapling::zip32::ExtendedFullViewingKey>>()
            };

            // Calculate the addresses
            let addresses = extfvks
                .iter()
                .map(|fvk| fvk.default_address().1)
                .collect::<Vec<sapling::PaymentAddress>>();

            // If extsks is of len 0, then this wallet is locked
            let zkeys_result = if extsks.len() == 0 {
                // Wallet is locked, so read only the viewing keys.
                extfvks
                    .iter()
                    .zip(addresses.iter())
                    .enumerate()
                    .map(|(i, (extfvk, payment_address))| {
                        let zk = WalletZKey::new_locked_hdkey(i as u32, extfvk.clone());
                        if zk.zaddress != *payment_address {
                            Err(io::Error::new(
                                ErrorKind::InvalidData,
                                "Payment address didn't match",
                            ))
                        } else {
                            Ok(zk)
                        }
                    })
                    .collect::<Vec<io::Result<WalletZKey>>>()
            } else {
                // Wallet is unlocked, read the spending keys as well
                extsks
                    .into_iter()
                    .zip(extfvks.into_iter().zip(addresses.iter()))
                    .enumerate()
                    .map(|(i, (extsk, (extfvk, payment_address)))| {
                        let zk = WalletZKey::new_hdkey(i as u32, extsk);
                        if zk.zaddress != *payment_address {
                            return Err(io::Error::new(
                                ErrorKind::InvalidData,
                                "Payment address didn't match",
                            ));
                        }

                        if zk.extfvk != extfvk {
                            return Err(io::Error::new(
                                ErrorKind::InvalidData,
                                "Full View key didn't match",
                            ));
                        }

                        Ok(zk)
                    })
                    .collect::<Vec<io::Result<WalletZKey>>>()
            };

            // Convert vector of results into result of vector, returning an error if any one of the keys failed the checks above
            zkeys_result.into_iter().collect::<io::Result<_>>()?
        } else {
            // After version 6, we read the WalletZKey structs directly
            Vector::read(&mut reader, |r| WalletZKey::read(r))?
        };

        let tkeys = if version <= 20 {
            let tkeys = Vector::read(&mut reader, |r| {
                let mut tpk_bytes = [0u8; 32];
                r.read_exact(&mut tpk_bytes)?;
                secp256k1::SecretKey::from_slice(&tpk_bytes)
                    .map_err(|e| io::Error::new(ErrorKind::InvalidData, e))
            })?;

            let taddresses = if version >= 4 {
                // Read the addresses
                Vector::read(&mut reader, |r| read_string(r))?
            } else {
                // Calculate the addresses
                tkeys
                    .iter()
                    .map(|sk| {
                        WalletTKey::address_from_prefix_sk(&config.base58_pubkey_address(), sk)
                    })
                    .collect()
            };

            tkeys
                .iter()
                .zip(taddresses.iter())
                .enumerate()
                .map(|(i, (sk, taddr))| WalletTKey::from_raw(sk, taddr, i as u32))
                .collect::<Vec<_>>()
        } else {
            // Read the TKeys
            Vector::read(&mut reader, |r| WalletTKey::read(r))?
        };

        Ok(Self {
            config: config.clone(),
            encrypted,
            enc_seed,
            nonce,
            seed: seed_bytes,
            zkeys,
            tkeys,
            okeys: vec![],
        })
    }

    pub fn get_all_extfvks(&self) -> Vec<sapling::zip32::ExtendedFullViewingKey> {
        self.zkeys.iter().map(|zk| zk.extfvk.clone()).collect()
    }

    pub fn have_sapling_spending_key(
        &self,
        extfvk: &sapling::zip32::ExtendedFullViewingKey,
    ) -> bool {
        self.zkeys
            .iter()
            .find(|zk| zk.extfvk == *extfvk)
            .map(|zk| zk.have_spending_key())
            .unwrap_or(false)
    }
}

impl<P: zcash_protocol::consensus::Parameters> ParseWithParam<(LightClientConfig<P>, u64)>
    for Keys<P>
{
    fn parse(p: &mut Parser, param: (LightClientConfig<P>, u64)) -> Result<Self> {
        let (config, version) = param;

        let keys = if version <= 14 {
            Keys::read_old(version, p, config)?
            // Keys::read(p, config).unwrap()
        } else {
            Keys::read(p, config)?
        };

        Ok(keys)
    }
}
