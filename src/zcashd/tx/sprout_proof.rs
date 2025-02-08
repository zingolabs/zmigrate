use anyhow::Result;

use crate::{ parse, Parser };

use super::{GrothProof, PHGRProof};

#[derive(Debug, Clone, PartialEq)]
pub enum SproutProof {
    PHGRProof(PHGRProof),
    GrothProof(GrothProof),
}

impl SproutProof {
    pub fn parse(parser: &mut Parser, use_groth: bool) -> Result<Self> where Self: Sized {
        if use_groth {
            let groth_proof = parse!(parser, "groth_proof")?;
            Ok(Self::GrothProof(groth_proof))
        } else {
            let phgr_proof = parse!(parser, "phgr_proof")?;
            Ok(Self::PHGRProof(phgr_proof))
        }
    }
}
