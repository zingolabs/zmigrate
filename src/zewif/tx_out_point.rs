use crate::TxId;

/// A reference to a previous transaction output.
#[derive(Debug, Clone)]
pub struct TxOutPoint {
    pub txid: TxId,
    pub index: u32,
}
