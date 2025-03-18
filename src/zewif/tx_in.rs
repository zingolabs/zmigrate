use crate::Data;

use super::TxOutPoint;

/// A transparent transaction input.
#[derive(Debug, Clone)]
pub struct TxIn {
    previous_output: TxOutPoint,
    /// Script signature for unlocking the previous output.
    script_sig: Data,
    sequence: u32,
}
