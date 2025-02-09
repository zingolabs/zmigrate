use anyhow::Result;
use crate::{ parse, Data, Parse, Parser };

use super::{ JSDescription, LockTime, SaplingBundle, TxIn, TxOut, TxVersion };

#[derive(Debug, Clone, PartialEq)]
pub struct WalletTx {
    version: TxVersion,
    vin: Vec<TxIn>,
    vout: Vec<TxOut>,
    lock_time: LockTime,
    expiry_height: u32,
    sapling_bundle: SaplingBundle,
    join_split: Option<Vec<JSDescription>>,
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

    pub fn lock_time(&self) -> &LockTime {
        &self.lock_time
    }

    pub fn expiry_height(&self) -> u32 {
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

        let vin;
        let vout;
        let lock_time;
        let expiry_height;
        let sapling_bundle: SaplingBundle;
        let join_split = None;
        if version.is_zip225() {
            todo!()
            // lock_time = parse!(p, "transaction lock time")?;
            // expiry_height = parse!(p, "transaction expiry height")?;
            // vin = parse!(p, "transaction inputs")?;
            // vout = parse!(p, "transaction outputs")?;
        } else {
            vin = parse!(p, "transaction inputs")?;
            vout = parse!(p, "transaction outputs")?;
            lock_time = parse!(p, "transaction lock time")?;
            expiry_height = if version.is_overwinter() || version.is_sapling() || version.is_future() {
                parse!(p, "transaction expiry height")?
            } else {
                0
            };
            if version.is_sapling() || version.is_future() {
                // println!("✅ Sapling bundle");
                sapling_bundle = parse!(p, "Sapling bundle")?;
            } else {
                // println!("❌ No Sapling bundle");
                sapling_bundle = SaplingBundle::default();
            }

            if version.number() >= 2 {
                // join_split = Some(parse!(p, Vec<JSDescription>, "JoinSplit descriptions")?);
            }
        }

        let rest = p.rest();
        Ok(Self {
            version,
            vin,
            vout,
            lock_time,
            expiry_height,
            sapling_bundle,
            join_split,
            rest,
        })
    }
}
