use anyhow::Result;

use crate::{parse, Parse, Parser};

use super::{ExtractedNoteCommitment, Nullifier, OrchardSignature, RedPallasVerificationKey, TransmittedNoteCiphertext, ValueCommitment};

#[derive(Debug, Clone, PartialEq)]
pub struct OrchardAction {
    nf: Nullifier,
    rk: RedPallasVerificationKey,
    cmx: ExtractedNoteCommitment,
    encrypted_note: TransmittedNoteCiphertext,
    cv_net: ValueCommitment,
    authorization: Option<OrchardSignature>,
}

impl Parse for OrchardAction {
    fn parse(p: &mut Parser) -> Result<Self> where Self: Sized {
        Ok(Self {
            nf: parse!(p, "Nullifier")?,
            rk: parse!(p, "RedPallasVerificationKey")?,
            cmx: parse!(p, "ExtractedNoteCommitment")?,
            encrypted_note: parse!(p, "TransmittedNoteCiphertext")?,
            cv_net: parse!(p, "ValueCommitment")?,
            authorization: None,
        })
    }
}
