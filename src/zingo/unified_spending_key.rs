use anyhow::{bail, anyhow, Result};

use crate::{blob, parse, sapling::SaplingExtendedSpendingKey, zingo::Typecode, CompactSize, Data, ParseWithParam, Parser};

use super::{Era, OrchardSpendingKey};

blob!(LegacyAccountPrivKey, 74);

#[derive(Debug, Clone)]
pub struct UnifiedSpendingKey {
    pub transparent: LegacyAccountPrivKey,
    pub sapling: SaplingExtendedSpendingKey,
    pub orchard: OrchardSpendingKey,
}

impl UnifiedSpendingKey {
    pub fn parse_with_size(p: &mut Parser) -> Result<Self> {
        let data = parse!(p, Data, "data")?;
        parse!(buf &data, UnifiedSpendingKey, param Era::Orchard, "data")
    }
}

impl ParseWithParam<Era> for UnifiedSpendingKey {
    fn parse(p: &mut Parser, expected_era: Era) -> Result<Self> {
        let id = parse!(p, u32, "id")?;
        let era = Era::try_from_id(id).ok_or_else(|| anyhow!("Invalid era id"))?;
        if era != expected_era {
            bail!("Era invalid");
        }

        let mut orchard = None;
        let mut sapling = None;
        let mut transparent = None;
        loop {
            let tc = parse!(p, Typecode, "typecode")?;
            let len = parse!(p, CompactSize, "key length")?.0;

            match tc {
                Typecode::Orchard => {
                    if len != 32 {
                        bail!("Orchard length mismatch: {}", len);
                    }

                    orchard = Some(parse!(p, OrchardSpendingKey, "OrchardSpendingKey")?);
                }
                Typecode::Sapling => {
                    if len != 169 {
                        bail!("Sapling length mismatch: {}", len);
                    }

                    sapling = Some(parse!(p, SaplingExtendedSpendingKey, "SaplingExtendedSpendingKey")?);
                }
                Typecode::P2pkh => {
                    if len != 74 {
                        bail!("P2pkh length mismatch: {}", len);
                    }

                    transparent = Some(parse!(p, LegacyAccountPrivKey, "LegacyAccountPrivKey")?);
                }
                _ => {
                    bail!("Typecode invalid");
                }
            }

            let has_orchard = orchard.is_some();
            let has_sapling = sapling.is_some();
            let has_transparent = transparent.is_some();

            if has_orchard && has_sapling && has_transparent {
                return Ok(UnifiedSpendingKey {
                    transparent: transparent.unwrap(),
                    sapling: sapling.unwrap(),
                    orchard: orchard.unwrap(),
                });
            }
        }
    }
}


// impl Parse for UnifiedSpendingKey {
//     fn parse(parser: &mut Parser) -> Result<Self> {
//         let transparent = parse!(parser, "transparent")?;
//         let sapling = parse!(parser, "sapling")?;
//         let orchard = parse!(parser, "orchard")?;
//         Ok(Self {
//             transparent,
//             sapling,
//             orchard,
//         })
//     }
// }

// pub(crate) fn legacy_sks_to_usk(
//     orchard_key: &OrchardSpendingKey,
//     sapling_key: &SaplingExtendedSpendingKey,
//     transparent_key: &ExtendedPrivKey,
// ) -> Result<UnifiedSpendingKey, KeyError> {
//     let mut usk_bytes = vec![];

//     // hard-coded Orchard Era ID due to `id()` being a private fn
//     usk_bytes.write_u32::<LittleEndian>(0xc2d6_d0b4)?;

//     CompactSize::write(
//         &mut usk_bytes,
//         usize::try_from(Typecode::Orchard).expect("typecode to usize should not fail"),
//     )?;
//     let orchard_key_bytes = orchard_key.to_bytes();
//     CompactSize::write(&mut usk_bytes, orchard_key_bytes.len())?;
//     usk_bytes.write_all(orchard_key_bytes)?;

//     CompactSize::write(
//         &mut usk_bytes,
//         usize::try_from(Typecode::Sapling).expect("typecode to usize should not fail"),
//     )?;
//     let sapling_key_bytes = sapling_key.to_bytes();
//     CompactSize::write(&mut usk_bytes, sapling_key_bytes.len())?;
//     usk_bytes.write_all(&sapling_key_bytes)?;

//     // the following code performs the same operations for calling `to_bytes()` on an AccountPrivKey in LRZ
//     let prefix = bip32::Prefix::XPRV;
//     let mut chain_code = [0u8; 32];
//     chain_code.copy_from_slice(&transparent_key.chain_code);
//     let attrs = bip32::ExtendedKeyAttrs {
//         depth: 4,
//         parent_fingerprint: [0xff, 0xff, 0xff, 0xff],
//         child_number: bip32::ChildNumber::new(0, true).expect("correct"),
//         chain_code,
//     };
//     // Add leading `0` byte
//     let mut key_bytes = [0u8; 33];
//     key_bytes[1..].copy_from_slice(transparent_key.private_key.as_ref());

//     let extended_key = bip32::ExtendedKey {
//         prefix,
//         attrs,
//         key_bytes,
//     };

//     let xprv_encoded = extended_key.to_string();
//     let account_tkey_bytes = bs58::decode(xprv_encoded)
//         .with_check(None)
//         .into_vec()
//         .expect("correct")
//         .split_off(bip32::Prefix::LENGTH);

//     CompactSize::write(
//         &mut usk_bytes,
//         usize::try_from(Typecode::P2pkh).expect("typecode to usize should not fail"),
//     )?;
//     CompactSize::write(&mut usk_bytes, account_tkey_bytes.len())?;
//     usk_bytes.write_all(&account_tkey_bytes)?;

//     UnifiedSpendingKey::from_bytes(Era::Orchard, &usk_bytes).map_err(|_| KeyError::KeyDecodingError)
// }
