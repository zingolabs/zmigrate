use std::collections::HashMap;

use anyhow::{bail, Context, Result};

use crate::{parse, Parse, Parser};

pub fn parse_compact_size(p: &mut Parser) -> Result<usize> {
    match parse!(p, u8, "compact size")? {
        0xfd =>
            parse!(p, u16, "compact size").map(|n| n as usize),
        0xfe =>
            parse!(p, u32, "compact size").map(|n| n as usize),
        0xff =>
            parse!(p, u64, "compact size").map(|n| n as usize),
        size => Ok(size as usize),
    }
}

pub fn parse_pair<T: Parse, U: Parse>(p: &mut Parser) -> Result<(T, U)> {
    let first = parse!(p, "first item of pair")?;
    let second = parse!(p, "second item of pair")?;
    Ok((first, second))
}

impl<T: Parse, U: Parse> Parse for (T, U) {
    fn parse(p: &mut Parser) -> Result<Self> {
        parse_pair(p)
    }
}

pub fn parse_fixed_length_vec<T: Parse>(p: &mut Parser, length: usize) -> Result<Vec<T>> {
    let mut items = Vec::with_capacity(length);
    for i in 0..length {
        items.push(parse!(p, format!("array item {} of {}", i, length - 1))?);
    }
    Ok(items)
}

pub fn parse_fixed_length_array<T: Parse, const N: usize>(p: &mut Parser) -> Result<[T; N]> {
    let items = parse_fixed_length_vec(p, N)?;
    let array: [T; N] = items.try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert Vec to fixed length array"))?;
    Ok(array)
}

pub fn parse_vec<T: Parse>(p: &mut Parser) -> Result<Vec<T>> {
    let length = parse_compact_size(p).context("array length")?;
    parse_fixed_length_vec(p, length)
}

impl<T: Parse, const N: usize> Parse for [T; N] {
    fn parse(p: &mut Parser) -> Result<Self> {
        parse_fixed_length_array(p)
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse(p: &mut Parser) -> Result<Self> {
        parse_vec(p)
    }
}

pub fn parse_map<K: Parse, V: Parse>(p: &mut Parser) -> Result<Vec<(K, V)>> {
    let length = parse_compact_size(p).context("map length")?;
    let mut items = Vec::with_capacity(length);
    for _ in 0..length {
        items.push(parse_pair::<K, V>(p).context("map item")?);
    }
    Ok(items)
}

pub fn parse_hashmap<K, V: Parse>(p: &mut Parser) -> Result<HashMap<K, V>>
    where K: Parse + Eq + std::hash::Hash
{
    Ok(parse_map::<K, V>(p)?.into_iter().collect())
}

impl<K: Parse, V: Parse> Parse for HashMap<K, V>
    where K: Parse + Eq + std::hash::Hash
{
    fn parse(p: &mut Parser) -> Result<Self> {
        parse_hashmap(p)
    }
}

/// A container that optionally holds a value, serialized with a presence flag followed by the value if present.                      | 1 byte (discriminant: 0x00 = absent, 0x01 = present) + serialized value `T` if present.
pub fn parse_optional<T: Parse>(p: &mut Parser) -> Result<Option<T>> {
    match parse!(p, u8, "optional discriminant")? {
        0x00 => Ok(None),
        0x01 => Ok(Some(parse!(p, "optional value")?)),
        discriminant => bail!("Invalid optional discriminant: {}", discriminant),
    }
}

impl<T: Parse> Parse for Option<T> {
    fn parse(p: &mut Parser) -> Result<Self> {
        parse_optional(p)
    }
}
