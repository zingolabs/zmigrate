#![allow(dead_code)]

use bip0039::Mnemonic;
use zcash_client_backend::proto::service::TreeState;
use zingolib::wallet::{data::{BlockData, WalletZecPriceInfo}, WalletOptions};

use super::WalletCapability;

#[derive(Debug)]
pub struct ZingoWallet {
    pub external_version: u64,
    pub chain_name: String,
    pub birthday: u64,
    pub mnemonic: Option<(Mnemonic, u32)>,
    pub wallet_options: WalletOptions,
    pub wallet_capability: WalletCapability,
    pub verified_tree: Option<TreeState>,
    pub price: WalletZecPriceInfo,
    pub last_100_blocks: Vec<BlockData>,
    pub remaining: usize,
}
