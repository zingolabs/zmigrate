use anyhow::{ Result, Context, bail };

use crate::Parseable;

impl Parseable for String {
    fn parse_type() -> &'static str {
        "String"
    }
    /// 1 byte (length) + bytes of the string
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let length = u8::parse(parser)? as usize;
        let bytes = parser.next(length).context("Parsing string")?;
        String::from_utf8(bytes.to_vec()).context("Parsing string")
    }
}

impl Parseable for bool {
    fn parse_type() -> &'static str {
        "bool"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let byte = u8::parse(parser).context("Parsing bool")?;
        match byte {
            0 => Ok(false),
            1 => Ok(true),
            _ => bail!("Invalid boolean value: {}", byte),
        }
    }
}

impl Parseable for u8 {
    fn parse_type() -> &'static str {
        "u8"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let bytes = parser.next(1).context("Parsing u8")?;
        Ok(bytes[0])
    }
}

impl Parseable for u16 {
    fn parse_type() -> &'static str {
        "u16"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<u16>();
        let bytes = parser.next(SIZE).context("Parsing u16")?;
        Ok(u16::from_le_bytes(bytes.try_into().context("Parsing u16")?))
    }
}

impl Parseable for u32 {
    fn parse_type() -> &'static str {
        "u32"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<u32>();
        let bytes = parser.next(SIZE).context("Parsing u32")?;
        Ok(u32::from_le_bytes(bytes.try_into().context("Parsing u32")?))
    }
}

impl Parseable for u64 {
    fn parse_type() -> &'static str {
        "u64"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<u64>();
        let bytes = parser.next(SIZE).context("Parsing u64")?;
        Ok(u64::from_le_bytes(bytes.try_into().context("Parsing u64")?))
    }
}

impl Parseable for i8 {
    fn parse_type() -> &'static str {
        "i8"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        let bytes = parser.next(1).context("Parsing i8")?;
        Ok(bytes[0] as i8)
    }
}

impl Parseable for i16 {
    fn parse_type() -> &'static str {
        "i16"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<i16>();
        let bytes = parser.next(SIZE).context("Parsing i16")?;
        Ok(i16::from_le_bytes(bytes.try_into().context("Parsing i16")?))
    }
}

impl Parseable for i32 {
    fn parse_type() -> &'static str {
        "i32"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<i32>();
        let bytes = parser.next(SIZE).context("Parsing i32")?;
        Ok(i32::from_le_bytes(bytes.try_into().context("Parsing i32")?))
    }
}

impl Parseable for i64 {
    fn parse_type() -> &'static str {
        "i64"
    }
    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        const SIZE: usize = std::mem::size_of::<i64>();
        let bytes = parser.next(SIZE).context("Parsing i64")?;
        Ok(i64::from_le_bytes(bytes.try_into().context("Parsing i64")?))
    }
}
