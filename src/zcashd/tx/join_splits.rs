use anyhow::Result;
use crate::{ parse, ParseWithParam, Parser };

use super::{ Ed25519Signature, Ed25519VerificationKey, JSDescription };

#[derive(Debug, Clone, PartialEq)]
pub struct JoinSplits {
    descriptions: Vec<JSDescription>,
    pub_key: Option<Ed25519VerificationKey>,
    sig: Option<Ed25519Signature>,
}

impl JoinSplits {
    pub fn descriptions(&self) -> &[JSDescription] {
        &self.descriptions
    }

    pub fn pub_key(&self) -> Option<&Ed25519VerificationKey> {
        self.pub_key.as_ref()
    }

    pub fn sig(&self) -> Option<&Ed25519Signature> {
        self.sig.as_ref()
    }
}

impl ParseWithParam<bool> for JoinSplits {
    fn parse(p: &mut Parser, use_groth: bool) -> Result<Self> {
        let descriptions = parse!(p, Vec<JSDescription>, param use_groth, "JoinSplit descriptions")?;
        if !descriptions.is_empty() {
            let pub_key = parse!(p, "JoinSplit public key")?;
            let sig = parse!(p, "JoinSplit signature")?;
            Ok(Self {
                descriptions,
                pub_key: Some(pub_key),
                sig: Some(sig),
            })
        } else {
            Ok(Self {
                descriptions,
                pub_key: None,
                sig: None,
            })
        }
    }
}
