use anyhow::Result;

use crate::Parseable;

impl Parseable for String {
    fn parse_type() -> &'static str {
        "String"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        parser.parse_utf8()
    }
}

impl Parseable for i64 {
    fn parse_type() -> &'static str {
        "i64"
    }

    fn parse(parser: &mut crate::Parser) -> Result<Self> where Self: Sized {
        parser.parse_i64()
    }
}
