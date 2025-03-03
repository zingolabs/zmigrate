use anyhow::Result;

use crate::{parse, u160, Parse, Parser};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ScriptId(pub u160);

impl Parse for ScriptId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let script_id = parse!(p, "script_id")?;
        Ok(ScriptId(script_id))
    }
}
