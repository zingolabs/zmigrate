use anyhow::{ Context, Result, bail };

use crate::Data;

pub trait Parseable {
    fn parse_type() -> &'static str;

    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized;

    fn parse_binary(buffer: &dyn AsRef<[u8]>) -> Result<Self> where Self: Sized {
        let mut parser = Parser::new(&buffer);
        let result = Self::parse(&mut parser)?;
        parser.check_finished()?;
        Ok(result)
    }
}

pub struct Parser<'a> {
    buffer: &'a [u8],
    offset: usize,
}

impl<'a> Parser<'a> {
    pub fn new(buffer: &'a dyn AsRef<[u8]>) -> Self {
        Self {
            buffer: buffer.as_ref(),
            offset: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn remaining(&self) -> usize {
        self.len() - self.offset
    }

    pub fn check_finished(&self) -> Result<()> {
        if self.offset < self.buffer.len() {
            bail!("Buffer has {} bytes left", self.remaining());
        }
        Ok(())
    }

    pub fn next(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.offset + n > self.buffer.len() {
            bail!("Buffer underflow at offset {}, needed {} bytes, only {} remaining", self.offset, n, self.remaining());
        }
        let bytes = &self.buffer[self.offset..self.offset + n];
        self.offset += n;
        Ok(bytes)
    }

    pub fn rest(&mut self) -> Data {
        Data::parse(self.remaining(), self).unwrap()
    }

    pub fn parse_compact_size(&mut self) -> Result<usize> {
        match u8::parse(self).context("Parsing compact size")? {
            0xfd =>
                u16::parse(self)
                    .map(|n| n as usize)
                    .context("Parsing compact size"),
            0xfe =>
                u32::parse(self)
                    .map(|n| n as usize)
                    .context("Parsing compact size"),
            0xff =>
                u64::parse(self)
                    .map(|n| n as usize)
                    .context("Parsing compact size"),
            size => Ok(size as usize),
        }
    }

    pub fn parse_item<T: Parseable>(&mut self) -> Result<T> {
        T::parse(self).with_context(|| format!("Parsing item of type '{}'", T::parse_type()))
    }

    pub fn parse_pair<T: Parseable, U: Parseable>(&mut self) -> Result<(T, U)> {
        let first = self.parse_item::<T>().context("Parsing first item of pair")?;
        let second = self.parse_item::<U>().context("Parsing second item of pair")?;
        Ok((first, second))
    }

    pub fn parse_fixed_length_array<T: Parseable>(&mut self, length: usize) -> Result<Vec<T>> {
        let mut items = Vec::with_capacity(length);
        for i in 0..length {
            items.push(self.parse_item::<T>().with_context(|| format!("Parsing array item {} of {}", i, length - 1))?);
        }
        Ok(items)
    }

    pub fn parse_array<T: Parseable>(&mut self) -> Result<Vec<T>> {
        let length = self.parse_compact_size().context("Parsing array length")?;
        self.parse_fixed_length_array(length)
    }

    pub fn parse_map<K: Parseable, V: Parseable>(&mut self) -> Result<Vec<(K, V)>> {
        let length = self.parse_compact_size().context("Parsing map length")?;
        let mut items = Vec::with_capacity(length);
        for _ in 0..length {
            items.push(self.parse_pair::<K, V>().context("Parsing map item")?);
        }
        Ok(items)
    }

    pub fn parse_hashmap<K, V: Parseable>(&mut self) -> Result<Vec<(K, V)>>
        where K: Parseable + Eq + std::hash::Hash
    {
        let map = self.parse_map::<K, V>()?;
        let mut hashmap = std::collections::HashMap::new();
        for (key, value) in map {
            hashmap.insert(key, value);
        }
        Ok(hashmap.into_iter().collect())
    }

    /// A container that optionally holds a value, serialized with a presence flag followed by the value if present.                      | 1 byte (discriminant: 0x00 = absent, 0x01 = present) + serialized value `T` if present.
    pub fn parse_optional<T: Parseable>(&mut self) -> Result<Option<T>> {
        match u8::parse(self).context("Parsing optional discriminant")? {
            0x00 => Ok(None),
            0x01 => Ok(Some(self.parse_item::<T>().context("Parsing optional value")?)),
            discriminant => bail!("Invalid optional discriminant: {}", discriminant),
        }
    }
}
