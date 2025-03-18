use crate::TxId;

/// A reference to a previous transaction output.
#[derive(Debug, Clone)]
pub struct TxOutPoint {
    txid: TxId,
    index: u32,
}
