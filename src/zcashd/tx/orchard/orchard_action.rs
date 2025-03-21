use anyhow::Result;

use crate::{Parse, Parser, parse, u256};

use super::{OrchardSignature, TransmittedNoteCiphertext};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAction {
    pub cv_net: u256,
    pub nf_old: u256,
    pub rk: u256,
    pub cmx: u256,
    pub encrypted_note: TransmittedNoteCiphertext,
    pub authorization: Option<OrchardSignature>,
}

impl Parse for OrchardAction {
    fn parse(p: &mut Parser) -> Result<Self>
    where
        Self: Sized,
    {
        let cv_net = parse!(p, "cv_net")?;
        let nf_old = parse!(p, "nf")?;
        let rk = parse!(p, "rk")?;
        let cmx = parse!(p, "cmx")?;
        let encrypted_note = parse!(p, "encrypted_note")?;
        Ok(Self {
            cv_net,
            nf_old,
            rk,
            cmx,
            encrypted_note,
            authorization: None,
        })
    }
}
