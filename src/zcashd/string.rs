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
