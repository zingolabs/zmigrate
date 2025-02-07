use std::collections::HashMap;

use anyhow::{bail, Context, Result};

use crate::{Parseable, Parser};

pub fn parse_compact_size(parser: &mut Parser) -> Result<usize> {
    match u8::parse(parser).context("Parsing compact size")? {
        0xfd =>
            u16::parse(parser)
                .map(|n| n as usize)
                .context("Parsing compact size"),
        0xfe =>
            u32::parse(parser)
                .map(|n| n as usize)
                .context("Parsing compact size"),
        0xff =>
            u64::parse(parser)
                .map(|n| n as usize)
                .context("Parsing compact size"),
        size => Ok(size as usize),
    }
}

pub fn parse_pair<T: Parseable, U: Parseable>(parser: &mut Parser) -> Result<(T, U)> {
    let first = T::parse(parser).context("Parsing first item of pair")?;
    let second = U::parse(parser).context("Parsing second item of pair")?;
    Ok((first, second))
}

pub fn parse_fixed_length_array<T: Parseable>(parser: &mut Parser, length: usize) -> Result<Vec<T>> {
    let mut items = Vec::with_capacity(length);
    for i in 0..length {
        items.push(T::parse(parser).with_context(|| format!("Parsing array item {} of {}", i, length - 1))?);
    }
    Ok(items)
}

pub fn parse_array<T: Parseable>(parser: &mut Parser) -> Result<Vec<T>> {
    let length = parse_compact_size(parser).context("Parsing array length")?;
    parse_fixed_length_array(parser, length)
}

pub fn parse_map<K: Parseable, V: Parseable>(parser: &mut Parser) -> Result<Vec<(K, V)>> {
    let length = parse_compact_size(parser).context("Parsing map length")?;
    let mut items = Vec::with_capacity(length);
    for _ in 0..length {
        items.push(parse_pair::<K, V>(parser).context("Parsing map item")?);
    }
    Ok(items)
}

pub fn parse_hashmap<K, V: Parseable>(parser: &mut Parser) -> Result<Vec<(K, V)>>
    where K: Parseable + Eq + std::hash::Hash
{
    let map = parse_map::<K, V>(parser)?;
    let mut hashmap = HashMap::new();
    for (key, value) in map {
        hashmap.insert(key, value);
    }
    Ok(hashmap.into_iter().collect())
}

/// A container that optionally holds a value, serialized with a presence flag followed by the value if present.                      | 1 byte (discriminant: 0x00 = absent, 0x01 = present) + serialized value `T` if present.
pub fn parse_optional<T: Parseable>(parser: &mut Parser) -> Result<Option<T>> {
    match u8::parse(parser).context("Parsing optional discriminant")? {
        0x00 => Ok(None),
        0x01 => Ok(Some(T::parse(parser).context("Parsing optional value")?)),
        discriminant => bail!("Invalid optional discriminant: {}", discriminant),
    }
}
