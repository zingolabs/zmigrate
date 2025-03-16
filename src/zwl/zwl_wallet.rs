use incrementalmerkletree::bridgetree::BridgeTree;
use orchard_old::tree::MerkleHashOrchard;
use zingolib::wallet::data::WalletZecPriceInfo;
use zl_zcash_client_backend::proto::service::TreeState;

use super::orchard_tree::MERKLE_DEPTH;
use super::{block::CompactBlockData, keys::Keys, transactions::WalletTxns};

#[allow(dead_code)]
#[derive(Debug)]
pub struct ZwlWallet {
    pub version: u64,
    pub keys: Keys<zcash_protocol::consensus::MainNetwork>,
    pub blocks: Vec<CompactBlockData>,
    pub transactions: WalletTxns,
    pub chain_name: String,
    pub wallet_options: crate::zwl::data::WalletOptions,
    pub birthday: u64,
    pub verified_tree: Option<TreeState>,
    pub orchard_witnesses: Option<BridgeTree<MerkleHashOrchard, MERKLE_DEPTH>>,
    pub price: WalletZecPriceInfo,
}
