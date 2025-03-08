use anyhow::{Result, bail};

use crate::{parse, CompactSize, Parse};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Typecode {
    /// A transparent P2PKH address, FVK, or IVK encoding as specified in [ZIP 316](https://zips.z.cash/zip-0316).
    P2pkh,
    /// A transparent P2SH address.
    ///
    /// This typecode cannot occur in a [`Ufvk`] or [`Uivk`].
    P2sh,
    /// A Sapling raw address, FVK, or IVK encoding as specified in [ZIP 316](https://zips.z.cash/zip-0316).
    Sapling,
    /// An Orchard raw address, FVK, or IVK encoding as specified in [ZIP 316](https://zips.z.cash/zip-0316).
    Orchard,
    /// An unknown or experimental typecode.
    Unknown(u32),
}

impl TryFrom<usize> for Typecode {
    type Error = anyhow::Error;

    fn try_from(typecode: usize) -> Result<Self, Self::Error> {
        match typecode {
            0x00 => Ok(Typecode::P2pkh),
            0x01 => Ok(Typecode::P2sh),
            0x02 => Ok(Typecode::Sapling),
            0x03 => Ok(Typecode::Orchard),
            0x04..=0x02000000 => Ok(Typecode::Unknown(typecode as u32)),
            0x02000001.. => bail!("Invalid typecode value: {}", typecode),
        }
    }
}

impl Parse for Typecode {
    fn parse(p: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let typecode = parse!(p, CompactSize, "typecode")?;
        Typecode::try_from(typecode.0)
    }
}
