use crate::Data;

use super::TxOutPoint;

/// A transparent transaction input.
#[derive(Debug, Clone)]
pub struct TxIn {
    pub previous_output: TxOutPoint,
    /// Script signature for unlocking the previous output.
    pub script_sig: Data,
    pub sequence: u32,
}
