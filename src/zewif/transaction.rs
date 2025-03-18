use crate::{BlockHeight, Data, TxId};

use super::{Attachments, JoinSplitDescription, OrchardActionDescription, SaplingOutputDescription, SaplingSpendDescription, TxIn, TxOut};


/// A transaction that can combine both transparent and shielded components.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The transaction id.
    pub txid: TxId,
    /// The raw transaction data, if known.
    pub raw: Option<Data>,
    /// The height at which the transaction was mined, if known.
    /// It is possible that if a rollback occurred just after the zeWIF
    /// export, the transaction could have been unmined, and possibly
    /// remined at a different height.
    pub mined_height: Option<BlockHeight>,

    // Design issue: do we want to parse out all of this? All wallets will
    // necessarily have code to parse a transaction. The only information
    // that is not redundant with the raw transaction encoding is the
    // *decrypted* note plaintexts (and it might be sufficient to just
    // indicate which output indices are expected to be decryptable with
    // which keys). I don't see the point of duplicating the raw data in a
    // different format (that still needs to be parsed!)
    // -- Daira-Emma

    /// Optional data for transparent inputs
    pub inputs: Option<Vec<TxIn>>,
    /// Optional data for transparent outputs
    pub outputs: Option<Vec<TxOut>>,
    /// Optional data for Sapling spends
    pub sapling_spends: Option<Vec<SaplingSpendDescription>>,
    /// Optional data for Sapling outputs
    pub sapling_outputs: Option<Vec<SaplingOutputDescription>>,
    /// Optional data for Orchard actions
    pub orchard_actions: Option<Vec<OrchardActionDescription>>,
    /// Optional data for Sprout JoinSplit descriptions
    pub sprout_joinsplits: Option<Vec<JoinSplitDescription>>,
    // Additional metadata such as confirmations or timestamp may be added here.
    pub attachments: Attachments,
}
