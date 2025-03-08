#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{parse, Data, ParseWithParam};
use anyhow::{bail, Result};

use super::{BlockData, WalletCapability};

#[derive(Debug)]
pub struct ZingoWallet {
    pub external_version: u64,
    pub wallet_capability: WalletCapability,

    pub last_100_blocks: Vec<BlockData>,

    pub remaining: usize,

    // The block at which this wallet was born. Rescans
    // will start from here.
    // birthday: u64,
    // /// The seed for the wallet, stored as a zip339 Mnemonic, and the account index.
    // /// Can be `None` in case of wallet without spending capability
    // /// or created directly from spending keys.
    // mnemonic: Option<(Mnemonic, u32)>,

    // /// Wallet options
    // pub wallet_options: WalletOptions,

    // /// Highest verified block
    // pub(crate) verified_tree: Option<TreeState>,

    // /// Progress of an outgoing transaction
    // send_progress: SendProgress,

    // /// The current price of ZEC. (time_fetched, price in USD)
    // pub price: WalletZecPriceInfo,

    // /// Local state needed to submit (compact)block-requests to the proxy
    // /// and interpret responses
    // pub transaction_context: TransactionContext,
}
