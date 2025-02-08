use std::collections::HashMap;

use anyhow::{bail, Context, Result};

use crate::{Parse, Parser};

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

pub fn parse_pair<T: Parse, U: Parse>(parser: &mut Parser) -> Result<(T, U)> {
    let first = Parse::parse(parser).context("Parsing first item of pair")?;
    let second = Parse::parse(parser).context("Parsing second item of pair")?;
    Ok((first, second))
}

impl<T: Parse, U: Parse> Parse for (T, U) {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parse_pair(parser)
    }
}

pub fn parse_fixed_length_vec<T: Parse>(parser: &mut Parser, length: usize) -> Result<Vec<T>> {
    let mut items = Vec::with_capacity(length);
    for i in 0..length {
        items.push(Parse::parse(parser).with_context(|| format!("Parsing array item {} of {}", i, length - 1))?);
    }
    Ok(items)
}

pub fn parse_fixed_length_array<T: Parse, const N: usize>(parser: &mut Parser) -> Result<[T; N]> {
    let items = parse_fixed_length_vec(parser, N)?;
    let array: [T; N] = items.try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert Vec to fixed length array"))?;
    Ok(array)
}

pub fn parse_vec<T: Parse>(parser: &mut Parser) -> Result<Vec<T>> {
    let length = parse_compact_size(parser).context("Parsing array length")?;
    parse_fixed_length_vec(parser, length)
}

impl<T: Parse, const N: usize> Parse for [T; N] {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parse_fixed_length_array(parser)
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parse_vec(parser)
    }
}

pub fn parse_map<K: Parse, V: Parse>(parser: &mut Parser) -> Result<Vec<(K, V)>> {
    let length = parse_compact_size(parser).context("Parsing map length")?;
    let mut items = Vec::with_capacity(length);
    for _ in 0..length {
        items.push(parse_pair::<K, V>(parser).context("Parsing map item")?);
    }
    Ok(items)
}

pub fn parse_hashmap<K, V: Parse>(parser: &mut Parser) -> Result<HashMap<K, V>>
    where K: Parse + Eq + std::hash::Hash
{
    Ok(parse_map::<K, V>(parser)?.into_iter().collect())
}

impl<K: Parse, V: Parse> Parse for HashMap<K, V>
    where K: Parse + Eq + std::hash::Hash
{
    fn parse(parser: &mut Parser) -> Result<Self> {
        parse_hashmap(parser)
    }
}

/// A container that optionally holds a value, serialized with a presence flag followed by the value if present.                      | 1 byte (discriminant: 0x00 = absent, 0x01 = present) + serialized value `T` if present.
pub fn parse_optional<T: Parse>(parser: &mut Parser) -> Result<Option<T>> {
    match u8::parse(parser).context("Parsing optional discriminant")? {
        0x00 => Ok(None),
        0x01 => Ok(Some(Parse::parse(parser).context("Parsing optional value")?)),
        discriminant => bail!("Invalid optional discriminant: {}", discriminant),
    }
}

impl<T: Parse> Parse for Option<T> {
    fn parse(parser: &mut Parser) -> Result<Self> {
        parse_optional(parser)
    }
}
