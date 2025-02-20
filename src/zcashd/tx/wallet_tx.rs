use std::collections::HashMap;

use anyhow::Result;
use crate::{ parse, u256, BranchId, Data, Parse, Parser, SaplingBundleV5 };

use super::{ ExpiryHeight, JSOutPoint, JoinSplits, LockTime, OrchardBundle, SaplingBundle, SaplingBundleV4, SaplingNoteData, SaplingOutPoint, SproutNoteData, TxIn, TxOut, TxVersion, SAPLING_TX_VERSION };


#[derive(Debug, Clone, PartialEq)]
pub struct WalletTx {
    // CTransaction
    pub version: TxVersion,

    pub consensus_branch_id: Option<BranchId>,

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

        let version: TxVersion = parse!(p, "version")?;

        let vin;
        let vout;
        let lock_time;
        let mut expiry_height = None;
        let sapling_bundle: SaplingBundle;
        let mut join_splits = None;
        let mut consensus_branch_id = None;
        let mut orchard_bundle = OrchardBundle(None);

        if version.is_zip225() {
            consensus_branch_id = Some(parse!(p, BranchId, "consensus_branch_id")?);
            lock_time = parse!(p, LockTime, "lock_time")?.as_option();
            expiry_height = parse!(p, ExpiryHeight, "expiry_height")?.as_option();
            vin = parse!(p, "vin")?;
            vout = parse!(p, "vout")?;
            let sapling_bundle_v5: SaplingBundleV5 = parse!(p, "sapling_bundle")?;
            sapling_bundle = SaplingBundle::V5(sapling_bundle_v5);

            orchard_bundle = parse!(p, "orchard_bundle")?;
        } else {
            vin = parse!(p, "vin")?;
            vout = parse!(p, "vout")?;
            lock_time = parse!(p, LockTime, "lock_time")?.as_option();
            if version.is_overwinter() || version.is_sapling() || version.is_future() {
                expiry_height = parse!(p, ExpiryHeight, "expiry_height")?.as_option();
            }

            let mut sapling_bundle_v4: SaplingBundleV4 = (version.is_sapling() || version.is_future())
                .then(|| parse!(p, "sapling_bundle"))
                .transpose()?
                .unwrap_or_default();

            if version.number >= 2 {
                let use_groth = version.is_overwinter() && version.number >= SAPLING_TX_VERSION;
                join_splits = Some(parse!(p, param use_groth, "join_splits")?);
            }

            if (version.is_sapling() || version.is_future()) && sapling_bundle_v4.have_actions() {
                let binding_sig = parse!(p, "binding_sig")?;
                sapling_bundle_v4.binding_sig = binding_sig;
            }

            sapling_bundle = SaplingBundle::V4(sapling_bundle_v4);
        }

        // CMerkleTx
        let hash_block = parse!(p, "hash_block")?;
        let merkle_branch = parse!(p, "merkle_branch")?;
        let index = parse!(p, "index")?;

        // CWalletTx
        let unused: Vec<i32> = parse!(p, "unused")?;
        assert!(unused.is_empty(), "unused field in CWalletTx is not empty");

        let map_value = parse!(p, "map_value")?;
        let map_sprout_note_data = parse!(p, "map_sprout_note_data")?;
        let order_form = parse!(p, "order_form")?;
        let time_received_is_tx_time = parse!(p, "time_received_is_tx_time")?;
        let time_received = parse!(p, "time_received")?;
        let from_me = parse!(p, "from_me")?;
        let is_spent = parse!(p, "is_spent")?;
        let sapling_note_data = parse!(p, "sapling_note_data")?;

        let unparsed_data = p.rest();
        assert!(unparsed_data.is_empty(), "unparsed_data in CWalletTx is not empty");

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
