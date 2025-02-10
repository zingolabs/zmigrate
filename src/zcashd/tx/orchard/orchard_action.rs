use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{ExtractedNoteCommitment, Nullifier, OrchardSignature, RedPallasVerificationKey, TransmittedNoteCiphertext, ValueCommitment};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAction {
    pub nf: Nullifier,
    pub rk: RedPallasVerificationKey,
    pub cmx: ExtractedNoteCommitment,
    pub encrypted_note: TransmittedNoteCiphertext,
    pub cv_net: ValueCommitment,
    pub authorization: Option<OrchardSignature>,
}

impl Parse for OrchardAction {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            nf: parse!(p, "nf")?,
            rk: parse!(p, "rk")?,
            cmx: parse!(p, "cmx")?,
            encrypted_note: parse!(p, "encrypted_note")?,
            cv_net: parse!(p, "cv_net")?,
            authorization: None,
        })
    }
}
