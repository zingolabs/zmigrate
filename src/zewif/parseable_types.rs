use anyhow::{Context, Result, bail};

use crate::{CompactSize, Parse, Parser, parse};

impl Parse for String {
    fn parse(p: &mut Parser) -> Result<Self> {
        let length = parse!(p, CompactSize, "string length")?;
        let bytes = parse!(p, bytes = *length, "string")?;
        String::from_utf8(bytes.to_vec()).context("string")
    }
}

pub fn parse_string<T>(p: &mut Parser) -> Result<String>
where
    T: Parse + TryInto<usize>,
    T::Error: std::error::Error + Send + Sync + 'static,
{
    let parsed = parse!(p, T, "string length")?;
    let length = parsed
        .try_into()
        .context("converting string length to usize")?;
    let bytes = parse!(p, bytes = length, "string data")?;
    String::from_utf8(bytes.to_vec()).context("string")
}

impl Parse for bool {
    fn parse(p: &mut Parser) -> Result<Self> {
        let byte = parse!(p, u8, "bool")?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => bail!("Invalid boolean value: {}", byte),
        }
    }
}

impl Parse for u8 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let bytes = p.next(1).context("u8")?;
        Ok(bytes[0])
    }
}

impl Parse for u16 {
    fn parse(p: &mut Parser) -> Result<Self> {
        const SIZE: usize = std::mem::size_of::<u16>();
        let bytes = p.next(SIZE).context("u16")?;
        Ok(u16::from_le_bytes(bytes.try_into().context("u16")?))
    }
}

impl Parse for u32 {
    fn parse(p: &mut Parser) -> Result<Self> {
        const SIZE: usize = std::mem::size_of::<u32>();
        let bytes = p.next(SIZE).context("u32")?;
        Ok(u32::from_le_bytes(bytes.try_into().context("u32")?))
    }
}

impl Parse for u64 {
    fn parse(p: &mut Parser) -> Result<Self> {
        const SIZE: usize = std::mem::size_of::<u64>();
        let bytes = p.next(SIZE).context("u64")?;
        Ok(u64::from_le_bytes(bytes.try_into().context("u64")?))
    }
}

impl Parse for i8 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let bytes = p.next(1).context("i8")?;
        Ok(bytes[0] as i8)
    }
}

impl Parse for i16 {
    fn parse(p: &mut Parser) -> Result<Self> {
        const SIZE: usize = std::mem::size_of::<i16>();
        let bytes = p.next(SIZE).context("i16")?;
        Ok(i16::from_le_bytes(bytes.try_into().context("i16")?))
    }
}

impl Parse for i32 {
    fn parse(p: &mut Parser) -> Result<Self> {
        const SIZE: usize = std::mem::size_of::<i32>();
        let bytes = p.next(SIZE).context("i32")?;
        Ok(i32::from_le_bytes(bytes.try_into().context("i32")?))
    }
}

impl Parse for i64 {
    fn parse(p: &mut Parser) -> Result<Self> {
        const SIZE: usize = std::mem::size_of::<i64>();
        let bytes = p.next(SIZE).context("i64")?;
        Ok(i64::from_le_bytes(bytes.try_into().context("i64")?))
    }
}

impl Parse for () {
    fn parse(_p: &mut Parser) -> Result<Self> {
        Ok(())
    }
}
