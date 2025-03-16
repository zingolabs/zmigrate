use std::{fmt, io};

use byteorder::{LittleEndian, ReadBytesExt};
use orchard_old::keys::{FullViewingKey, Scope, SpendingKey};
use zcash_encoding::{Optional, Vector};
use zcash_keys::address::UnifiedAddress;

use orchard_new::Address as NewAddress;
use orchard_old::Address as OldAddress;

#[derive(PartialEq, Debug, Clone)]
pub enum WalletOKeyType {
    HdKey = 0,
    ImportedSpendingKey = 1,
    ImportedFullViewKey = 2,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct WalletOKey {
    pub locked: bool,

    pub keytype: WalletOKeyType,
    pub sk: Option<SpendingKey>,
    pub fvk: FullViewingKey,
    pub unified_address: UnifiedAddress,

    // If this is a HD key, what is the key number
    pub hdkey_num: Option<u32>,

    // If locked, the encrypted private key is stored here
    pub enc_key: Option<Vec<u8>>,
    pub nonce: Option<Vec<u8>>,
}

impl WalletOKey {
    pub fn serialized_version() -> u8 {
        1
    }

    pub fn read<R: ReadBytesExt>(mut reader: R) -> io::Result<Self> {
        let version = reader.read_u8()?;
        assert!(version <= Self::serialized_version());

        // Read orchard key type
        let keytype = match reader.read_u32::<LittleEndian>()? {
            0 => Ok(WalletOKeyType::HdKey),
            1 => Ok(WalletOKeyType::ImportedSpendingKey),
            2 => Ok(WalletOKeyType::ImportedFullViewKey),
            n => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Invalid okey type {}", n),
            )),
        }?;

        // read if key is locked
        let locked = reader.read_u8()? > 0;

        // If HD derived, read the key index
        let hdkey_num = Optional::read(&mut reader, |r| r.read_u32::<LittleEndian>())?;

        // read address fvk
        let fvk = FullViewingKey::read(&mut reader)?;

        // read sk if available (read as 32 bytes)
        let sk = Optional::read(&mut reader, |r| {
            let mut bytes = [0u8; 32];
            r.read_exact(&mut bytes)?;
            Ok(SpendingKey::from_bytes(bytes).unwrap())
        })?;

        // Derive unified address (orchard only) from fvk
        let old_address: orchard_old::Address = fvk.address_at(0u64, Scope::External);

        let new_address = NewAddress::from_old(old_address);
        let unified_address = UnifiedAddress::from_receivers(Some(new_address), None, None)
            .expect("Failed to construct unified address");

        // read "possible" encrypted key
        let enc_key = Optional::read(&mut reader, |r| Vector::read(r, |r| r.read_u8()))?;

        // read "possible" nouce used in key encryption
        let nonce = Optional::read(&mut reader, |r| Vector::read(r, |r| r.read_u8()))?;

        Ok(Self {
            locked,
            keytype,
            sk,
            fvk,
            unified_address,
            hdkey_num,
            enc_key,
            nonce,
        })
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for WalletOKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.keytype {
            WalletOKeyType::HdKey => {
                writeln!(f, "Type: HD key").unwrap();
            }
            WalletOKeyType::ImportedSpendingKey => {
                writeln!(f, "Type: Imported spending key").unwrap();
            }
            WalletOKeyType::ImportedFullViewKey => {
                writeln!(f, "Type: Imported full view key").unwrap();
            }
            _ => {
                writeln!(f, "Type: Unknown").unwrap();
            }
        }

        match self.locked {
            true => {
                writeln!(f, "Status: Encrypted").unwrap();
            }
            false => {
                writeln!(f, "Status: Decrypted").unwrap();
            }
        }

        if let Some(sk) = &self.sk {
            writeln!(f, "{:?}", sk).unwrap();
        }

        writeln!(f, "{:?}", self.fvk).unwrap();

        writeln!(f, "{:?}", self.unified_address).unwrap();

        Ok(())
    }
}

trait MyFrom<T> {
    fn from_old(old: T) -> Self;
}

impl MyFrom<OldAddress> for NewAddress {
    fn from_old(old: OldAddress) -> Self {
        Self::from_raw_address_bytes(&old.to_raw_address_bytes()).unwrap()
    }
}
