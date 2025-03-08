use anyhow::Result;

use crate::{parse, parse_fixed_length_vec, u256, Blob64, Parse, Parser};

use super::{
    super::{Amount, GrothProof},
    OutputDescription, OutputV5, SpendDescription, SpendV5,
};

#[derive(Debug, Clone, PartialEq)]
pub struct SaplingBundleV5 {
    pub shielded_spends: Vec<SpendDescription>,
    pub shielded_outputs: Vec<OutputDescription>,
    pub value_balance: Amount,
    pub authorization: Option<Blob64>,
}

impl Parse for SaplingBundleV5 {
    fn parse(p: &mut Parser) -> Result<Self> {
        let sd_v5s = parse!(p, Vec<SpendV5>, "spends")?;
        let od_v5s = parse!(p, Vec<OutputV5>, "outputs")?;
        let n_spends = sd_v5s.len();
        let n_outputs = od_v5s.len();
        let value_balance = if n_spends > 0 || n_outputs > 0 {
            parse!(p, "amount")?
        } else {
            Amount::default()
        };

        let anchor = if n_spends > 0 {
            Some(parse!(p, u256, "anchor")?)
        } else {
            None
        };

        let v_spend_proofs: Vec<GrothProof> = parse_fixed_length_vec(p, n_spends)?;
        let v_spend_auth_sigs: Vec<Blob64> = parse_fixed_length_vec(p, n_spends)?;
        let v_output_proofs: Vec<GrothProof> = parse_fixed_length_vec(p, n_outputs)?;

        let binding_sig: Option<Blob64> = if n_spends > 0 || n_outputs > 0 {
            Some(parse!(p, "binding_sig")?)
        } else {
            None
        };

        let shielded_spends = sd_v5s
            .into_iter()
            .zip(v_spend_proofs.into_iter().zip(v_spend_auth_sigs))
            .map(|(sd_5, (zkproof, spend_auth_sig))| {
                // the following `unwrap` is safe because we know n_spends > 0.
                sd_5.into_spend_description(anchor.clone().unwrap(), zkproof, spend_auth_sig)
            })
            .collect();

        let shielded_outputs = od_v5s
            .into_iter()
            .zip(v_output_proofs)
            .map(|(od_5, zkproof)| od_5.into_output_description(zkproof))
            .collect();

        Ok(Self {
            shielded_spends,
            shielded_outputs,
            value_balance,
            authorization: binding_sig,
        })
    }
}
