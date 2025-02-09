use anyhow::Result;
use crate::{ parse, Data, Parse, Parser };

use super::{ ExpiryHeight, JoinSplits, LockTime, SaplingBundle, TxIn, TxOut, TxVersion, SAPLING_TX_VERSION };


#[derive(Debug, Clone, PartialEq)]
pub struct WalletTx {
    version: TxVersion,
    vin: Vec<TxIn>,
    vout: Vec<TxOut>,
    lock_time: Option<LockTime>,
    expiry_height: Option<ExpiryHeight>,
    sapling_bundle: SaplingBundle,
    join_splits: Option<JoinSplits>,
    rest: Data,
}

impl WalletTx {
    pub fn version(&self) -> &TxVersion {
        &self.version
    }

    pub fn vin(&self) -> &[TxIn] {
        &self.vin
    }

    pub fn vout(&self) -> &[TxOut] {
        &self.vout
    }

    pub fn lock_time(&self) -> Option<LockTime> {
        self.lock_time
    }

    pub fn expiry_height(&self) -> Option<ExpiryHeight> {
        self.expiry_height
    }

    pub fn sapling_bundle(&self) -> &SaplingBundle {
        &self.sapling_bundle
    }

    pub fn rest(&self) -> &Data {
        &self.rest
    }
}

impl Parse for WalletTx {
    fn parse(p: &mut Parser) -> Result<Self> {
        let version: TxVersion = parse!(p, "transaction version")?;

        let mut vin = Vec::new();
        let mut vout = Vec::new();
        let mut lock_time = None;
        let mut expiry_height = None;
        let mut sapling_bundle: SaplingBundle = SaplingBundle::default();
        let mut join_splits = None;
        if version.is_zip225() {
            // lock_time = parse!(p, "transaction lock time")?;
            // expiry_height = parse!(p, "transaction expiry height")?;
            // vin = parse!(p, "transaction inputs")?;
            // vout = parse!(p, "transaction outputs")?;
            todo!()
        } else {
            vin = parse!(p, "transaction inputs")?;
            vout = parse!(p, "transaction outputs")?;
            lock_time = parse!(p, LockTime, "transaction lock time")?.as_option();
            if version.is_overwinter() || version.is_sapling() || version.is_future() {
                expiry_height = parse!(p, ExpiryHeight, "transaction expiry height")?.as_option();
            }

            sapling_bundle = (version.is_sapling() || version.is_future())
                .then(|| parse!(p, "Sapling bundle"))
                .transpose()?
                .unwrap_or_default();

            if version.number() >= 2 {
                let use_groth = version.is_overwinter() && version.number >= SAPLING_TX_VERSION;
                join_splits = Some(parse!(p, param use_groth, "JoinSplits")?);
            }

            // if (version.is_sapling() || version.is_future()) && sapling_bundle.have_actions() {
            //     let binding_sig = parse!(p, "Sapling bundle signature")?;
            //     sapling_bundle.set_binding_sig(binding_sig);
            // }
        }

        let rest = p.rest();
        Ok(Self {
            version,
            vin,
            vout,
            lock_time,
            expiry_height,
            sapling_bundle,
            join_splits,
            rest,
        })
    }
}
