use std::collections::HashMap;

use anyhow::Result;

use crate::{parse, Blob64, ClientVersion, Parse, Parser};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardTxMeta {
    pub version: ClientVersion,
    pub action_data: HashMap<u32, Blob64>,
    pub actions_spending_my_nodes: Vec<u32>,
}

impl Parse for OrchardTxMeta {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            version: parse!(parser, "version")?,
            action_data: parse!(parser, "action_data")?,
            actions_spending_my_nodes: parse!(parser, "actions_spending_my_nodes")?,
        })
    }
}
