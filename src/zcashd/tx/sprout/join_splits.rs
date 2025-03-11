use crate::{blob, parse, ParseWithParam, Parser};
use anyhow::Result;

use super::JSDescription;

blob!(Ed25519Signature, 64);
blob!(Ed25519VerificationKey, 32);

#[derive(Debug, Clone, PartialEq)]
pub struct JoinSplits {
    pub descriptions: Vec<JSDescription>,
    pub pub_key: Option<Ed25519VerificationKey>,
    pub sig: Option<Ed25519Signature>,
}

impl ParseWithParam<bool> for JoinSplits {
    fn parse(p: &mut Parser, use_groth: bool) -> Result<Self> {
        let descriptions = parse!(p, Vec<JSDescription>, param = use_groth, "descriptions")?;
        if !descriptions.is_empty() {
            Ok(Self {
                descriptions,
                pub_key: Some(parse!(p, "pub_key")?),
                sig: Some(parse!(p, "sig")?),
            })
        } else {
            Ok(Self { descriptions, pub_key: None, sig: None })
        }
    }
}
