use anyhow::Result;

use crate::{ parse, GrothProof, ParseWithParam, Parser };

use super::PHGRProof;

#[derive(Debug, Clone, PartialEq)]
pub enum SproutProof {
    PHGRProof(PHGRProof),
    GrothProof(GrothProof),
}

impl ParseWithParam<bool> for SproutProof {
    fn parse(p: &mut Parser, use_groth: bool) -> Result<Self> {
        if use_groth {
            let groth_proof = parse!(p, "groth_proof")?;
            Ok(Self::GrothProof(groth_proof))
        } else {
            let phgr_proof = parse!(p, "phgr_proof")?;
            Ok(Self::PHGRProof(phgr_proof))
        }
    }
}
