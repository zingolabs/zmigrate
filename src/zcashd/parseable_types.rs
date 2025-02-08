use anyhow::{ Result, Context, bail };

use crate::{parse, Data, Parse, Parser};

use super::parse_compact_size;

impl Parse for String {
    /// 1 byte (length) + bytes of the string
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let length = parse!(u8, parser, "string length")? as usize;
        let bytes = parse!(bytes length, parser, "Parsing string")?;
        String::from_utf8(bytes.to_vec()).context("Parsing string")
    }
}

impl Parse for bool {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let byte = parse!(u8, parser, "Parsing bool")?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => bail!("Invalid boolean value: {}", byte),
        }
    }
}

impl Parse for u8 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let bytes = parser.next(1).context("Parsing u8")?;
        Ok(bytes[0])
    }
}

impl Parse for u16 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<u16>();
        let bytes = parser.next(SIZE).context("Parsing u16")?;
        Ok(u16::from_le_bytes(bytes.try_into().context("Parsing u16")?))
    }
}

impl Parse for u32 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<u32>();
        let bytes = parser.next(SIZE).context("Parsing u32")?;
        Ok(u32::from_le_bytes(bytes.try_into().context("Parsing u32")?))
    }
}

impl Parse for u64 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<u64>();
        let bytes = parser.next(SIZE).context("Parsing u64")?;
        Ok(u64::from_le_bytes(bytes.try_into().context("Parsing u64")?))
    }
}

impl Parse for i8 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let bytes = parser.next(1).context("Parsing i8")?;
        Ok(bytes[0] as i8)
    }
}

impl Parse for i16 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<i16>();
        let bytes = parser.next(SIZE).context("Parsing i16")?;
        Ok(i16::from_le_bytes(bytes.try_into().context("Parsing i16")?))
    }
}

impl Parse for i32 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<i32>();
        let bytes = parser.next(SIZE).context("Parsing i32")?;
        Ok(i32::from_le_bytes(bytes.try_into().context("Parsing i32")?))
    }
}

impl Parse for i64 {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<i64>();
        let bytes = parser.next(SIZE).context("Parsing i64")?;
        Ok(i64::from_le_bytes(bytes.try_into().context("Parsing i64")?))
    }
}

impl Parse for Data {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        let len = parse_compact_size(parser).context("Parsing Data length")?;
        Self::parse_len(len as usize, parser)
    }
}
