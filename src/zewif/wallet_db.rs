use std::collections::HashMap;

use bc_components::{Digest, ARID};
use bc_envelope::prelude::*;

use crate::TxId;

use super::{Attachments, Transaction, Wallet};


/// Represents a wallet database, the top level of the interchange format
/// hierarchy, which can contain multiple wallets and a global transaction
/// history.
#[derive(Debug, Clone)]
pub struct WalletDB {
    wallets: HashMap<ARID, Wallet>,
    transactions: HashMap<TxId, Transaction>,
    attachments: Attachments,
}
