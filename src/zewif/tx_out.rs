use crate::{Amount, Data};

/// A transparent transaction output.
#[derive(Debug, Clone)]
pub struct TxOut {
    value: Amount,
    script_pubkey: Data,
}
