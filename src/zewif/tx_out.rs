use crate::{Amount, Data};

/// A transparent transaction output.
#[derive(Debug, Clone)]
pub struct TxOut {
    pub value: Amount,
    pub script_pubkey: Data,
}
