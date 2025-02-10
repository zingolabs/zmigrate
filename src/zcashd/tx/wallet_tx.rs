use std::collections::HashMap;

use anyhow::Result;
use crate::{ parse, u256, Data, Parse, Parser };

use super::{ ExpiryHeight, JSOutPoint, JoinSplits, LockTime, SaplingBundle, SaplingNoteData, SaplingOutPoint, SproutNoteData, TxIn, TxOut, TxVersion, SAPLING_TX_VERSION };


#[derive(Debug, Clone, PartialEq)]
pub struct WalletTx {
    // CTransaction
    pub version: TxVersion,

    pub consensus_branch_id: Option<u32>,

    pub vin: Vec<TxIn>,
    pub vout: Vec<TxOut>,
    pub lock_time: Option<LockTime>,
    pub expiry_height: Option<ExpiryHeight>,
    pub sapling_bundle: SaplingBundle,

    pub orchard_bundle: OrchardBundle,

    pub join_splits: Option<JoinSplits>,

    // CMerkleTx
    pub hash_block: u256,
    pub merkle_branch: Vec<u256>,
    pub index: i32,

    // CWalletTx
    pub map_value: HashMap<String, String>,
    pub map_sprout_note_data: HashMap<JSOutPoint, SproutNoteData>,
    pub order_form: Vec<(String, String)>,
    pub time_received_is_tx_time: i32,
    pub time_received: i32,
    pub from_me: bool,
    pub is_spent: bool,
    pub sapling_note_data: HashMap<SaplingOutPoint, SaplingNoteData>,

    pub unparsed_data: Data,
}

impl Parse for WalletTx {
    fn parse(p: &mut Parser) -> Result<Self> {
        // CTransaction

        let version: TxVersion = parse!(p, "transaction version")?;

        let mut vin = Vec::new();
        let mut vout = Vec::new();
        let mut lock_time;
        let mut expiry_height = None;
        let mut sapling_bundle: SaplingBundle = SaplingBundle::default();
        let mut join_splits = None;
        let mut consensus_branch_id = None;

        if version.is_zip225() {
            consensus_branch_id = parse!(p, "consensus branch id")?;
            lock_time = parse!(p, "transaction lock time")?;
            expiry_height = parse!(p, "transaction expiry height")?;
            vin = parse!(p, "transaction inputs")?;
            vout = parse!(p, "transaction outputs")?;
            sapling_bundle = parse!(p, "Sapling bundle")?;
            orchard_bundle = parse!(p, "Orchard bundle")?;
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

            if version.number >= 2 {
                let use_groth = version.is_overwinter() && version.number >= SAPLING_TX_VERSION;
                join_splits = Some(parse!(p, param use_groth, "JoinSplits")?);
            }

            if (version.is_sapling() || version.is_future()) && sapling_bundle.have_actions() {
                let binding_sig = parse!(p, "Sapling bundle signature")?;
                sapling_bundle.binding_sig = binding_sig;
            }
        }

        // CMerkleTx
        let hash_block = parse!(p, "hash block")?;
        let merkle_branch = parse!(p, "merkle branch")?;
        let index = parse!(p, "index")?;

        // CWalletTx
        let unused: Vec<i32> = parse!(p, "unused")?;
        assert!(unused.is_empty(), "unused field in CWalletTx is not empty");

        let map_value = parse!(p, "map value")?;
        let map_sprout_note_data = parse!(p, "map sprout note data")?;
        let order_form = parse!(p, "order form")?;
        let time_received_is_tx_time = parse!(p, "time received is tx time")?;
        let time_received = parse!(p, "time received")?;
        let from_me = parse!(p, "from me")?;
        let is_spent = parse!(p, "is spent")?;
        let sapling_note_data = parse!(p, "sapling note data")?;

        let unparsed_data = p.rest();
        assert!(unparsed_data.is_empty(), "unparsed data in CWalletTx is not empty");
        Ok(Self {
            // CTransaction
            version,
            consensus_branch_id,
            vin,
            vout,
            lock_time,
            expiry_height,
            sapling_bundle,
            orchard_bundle,
            join_splits,

            // CMerkleTx
            hash_block,
            merkle_branch,
            index,

            // CWalletTx
            map_value,
            map_sprout_note_data,
            order_form,
            time_received_is_tx_time,
            time_received,
            from_me,
            is_spent,
            sapling_note_data,

            unparsed_data,
        })
    }
}
