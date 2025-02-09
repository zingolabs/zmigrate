use anyhow::{ Result, bail };

use crate::Data;

#[macro_export]
macro_rules! parse {
    (buf $buf:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::Parse>::parse_buf($buf),
            || format!("Parsing {}", $context)
        )
    };
    (buf $buf:expr, $type:ty, param $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::ParseWithParam<_>>::parse_buf($buf, $param),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, $type:ty, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::Parse>::parse($parser),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, $type:ty, param $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            <$type as $crate::ParseWithParam<_>>::parse($parser, $param),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, bytes $length:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::Parser::next($parser, $length),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, data $length:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::Data::parse_len($parser, $length),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::Parse::parse($parser),
            || format!("Parsing {}", $context)
        )
    };
    ($parser:expr, param $param:expr, $context:expr) => {
        ::anyhow::Context::with_context(
            $crate::ParseWithParam::parse($parser, $param),
            || format!("Parsing {}", $context)
        )
    };
}

pub trait Parse {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized;

    fn parse_buf(buf: &dyn AsRef<[u8]>) -> Result<Self> where Self: Sized {
        let mut parser = Parser::new(&buf);
        let result = Self::parse(&mut parser)?;
        parser.check_finished()?;
        Ok(result)
    }
}

pub trait ParseWithParam<P> {
    fn parse(parser: &mut Parser, param: P) -> Result<Self> where Self: Sized;

    fn parse_buf(buf: &dyn AsRef<[u8]>, param: P) -> Result<Self> where Self: Sized {
        let mut parser = Parser::new(&buf);
        let result = Self::parse(&mut parser, param)?;
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
        Data::parse_len(self, self.remaining()).unwrap()
    }
}
