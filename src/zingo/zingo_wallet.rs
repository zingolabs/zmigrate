#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{parse, ParseWithParam};
use anyhow::{bail, Result};

pub struct ZingoWallet {
    // The block at which this wallet was born. Rescans
    // will start from here.
    birthday: u64,
    // /// The seed for the wallet, stored as a zip339 Mnemonic, and the account index.
    // /// Can be `None` in case of wallet without spending capability
    // /// or created directly from spending keys.
    // mnemonic: Option<(Mnemonic, u32)>,

    // /// The last 100 blocks, used if something gets re-orged
    // pub last_100_blocks: Vec<BlockData>,

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

impl ZingoWallet {
    pub const fn serialized_version() -> u64 {
        31
    }
}

pub struct ZingoConfig;

impl ParseWithParam<ZingoConfig> for ZingoWallet {
    fn parse(parser: &mut crate::Parser, config: ZingoConfig) -> Result<Self>
    where
        Self: Sized,
    {
        // let external_version = reader.read_u64::<LittleEndian>()?;
        let external_version = parse!(parser, u64, "wallet version")?;
        if external_version > Self::serialized_version() {
            bail!(
                "Don't know how to read wallet version {}. Do you have the latest version?",
                external_version
            );
        }

        // info!("Reading wallet version {}", external_version);
        // let mut wallet_capability = WalletCapability::read(&mut reader, config.chain)?;

        // let mut blocks = Vector::read(&mut reader, |r| BlockData::read(r))?;
        // if external_version <= 14 {
        //     // Reverse the order, since after version 20, we need highest-block-first
        //     // TODO: Consider order between 14 and 20.
        //     blocks = blocks.into_iter().rev().collect();
        // }

        // let transactions = if external_version <= 14 {
        //     TxMap::read_old(&mut reader, &wallet_capability)
        // } else {
        //     TxMap::read(&mut reader, &wallet_capability)
        // }?;

        // let chain_name = utils::read_string(&mut reader)?;

        // if chain_name != config.chain.to_string() {
        //     return Err(Error::new(
        //         ErrorKind::InvalidData,
        //         format!(
        //             "Wallet chain name {} doesn't match expected {}",
        //             chain_name, config.chain
        //         ),
        //     ));
        // }

        // let wallet_options = if external_version <= 23 {
        //     WalletOptions::default()
        // } else {
        //     WalletOptions::read(&mut reader)?
        // };

        // let birthday = reader.read_u64::<LittleEndian>()?;

        // if external_version <= 22 {
        //     let _sapling_tree_verified = if external_version <= 12 {
        //         true
        //     } else {
        //         reader.read_u8()? == 1
        //     };
        // }

        // let verified_tree = if external_version <= 21 {
        //     None
        // } else {
        //     Optional::read(&mut reader, |r| {
        //         use prost::Message;

        //         let buf = Vector::read(r, |r| r.read_u8())?;
        //         TreeState::decode(&buf[..])
        //             .map_err(|e| io::Error::new(ErrorKind::InvalidData, e.to_string()))
        //     })?
        // };

        // let price = if external_version <= 13 {
        //     WalletZecPriceInfo::default()
        // } else {
        //     WalletZecPriceInfo::read(&mut reader)?
        // };

        // let _orchard_anchor_height_pairs = if external_version == 25 {
        //     Vector::read(&mut reader, |r| {
        //         let mut anchor_bytes = [0; 32];
        //         r.read_exact(&mut anchor_bytes)?;
        //         let block_height = BlockHeight::from_u32(r.read_u32::<LittleEndian>()?);
        //         Ok((
        //             Option::<orchard::Anchor>::from(orchard::Anchor::from_bytes(anchor_bytes))
        //                 .ok_or(Error::new(ErrorKind::InvalidData, "Bad orchard anchor"))?,
        //             block_height,
        //         ))
        //     })?
        // } else {
        //     Vec::new()
        // };

        // let seed_bytes = Vector::read(&mut reader, |r| r.read_u8())?;
        // let mnemonic = if !seed_bytes.is_empty() {
        //     let account_index = if external_version >= 28 {
        //         reader.read_u32::<LittleEndian>()?
        //     } else {
        //         0
        //     };
        //     Some((
        //         Mnemonic::from_entropy(seed_bytes)
        //             .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?,
        //         account_index,
        //     ))
        // } else {
        //     None
        // };

        // // Derive unified spending key from seed and override temporary USK if wallet is pre v29.
        // //
        // // UnifiedSpendingKey is initially incomplete for old wallet versions.
        // // This is due to the legacy transparent extended private key (ExtendedPrivKey) not containing all information required for BIP0032.
        // // There is also the issue that the legacy transparent private key is derived an extra level to the external scope.
        // if external_version < 29 {
        //     if let Some(mnemonic) = mnemonic.as_ref() {
        //         wallet_capability.unified_key_store = UnifiedKeyStore::Spend(Box::new(
        //             UnifiedSpendingKey::from_seed(
        //                 &config.chain,
        //                 &mnemonic.0.to_seed(""),
        //                 AccountId::ZERO,
        //             )
        //             .map_err(|e| {
        //                 Error::new(
        //                     ErrorKind::InvalidData,
        //                     format!(
        //                         "Failed to derive unified spending key from stored seed bytes. {}",
        //                         e
        //                     ),
        //                 )
        //             })?,
        //         ));
        //     } else if let UnifiedKeyStore::Spend(_) = &wallet_capability.unified_key_store {
        //         return Err(io::Error::new(
        //             ErrorKind::Other,
        //             "loading from legacy spending keys with no seed phrase to recover",
        //         ));
        //     }
        // }

        // info!("Keys in this wallet:");
        // match &wallet_capability.unified_key_store {
        //     UnifiedKeyStore::Spend(_) => {
        //         info!("  - orchard spending key");
        //         info!("  - sapling extended spending key");
        //         info!("  - transparent extended private key");
        //     }
        //     UnifiedKeyStore::View(ufvk) => {
        //         if ufvk.orchard().is_some() {
        //             info!("  - orchard full viewing key");
        //         }
        //         if ufvk.sapling().is_some() {
        //             info!("  - sapling diversifiable full viewing key");
        //         }
        //         if ufvk.transparent().is_some() {
        //             info!("  - transparent extended public key");
        //         }
        //     }
        //     UnifiedKeyStore::Empty => info!("  - no keys found"),
        // }

        // // this initialization combines two types of data
        // let transaction_context = TransactionContext::new(
        //     // Config data could be used differently based on the circumstances
        //     // hardcoded?
        //     // entered at init by user?
        //     // stored on disk in a separate location and connected by a descendant library (such as zingo-mobile)?
        //     config,
        //     // Saveable Arc data
        //     //   - Arcs allow access between threads.
        //     //   - This data is loaded from the wallet file and but needs multithreaded access during sync.
        //     wallet_capability,
        //     transactions,
        // );

        // let lw = Self {
        //     last_100_blocks: blocks,
        //     mnemonic,
        //     wallet_options: wallet_options,
        //     birthday,
        //     verified_tree: verified_tree,
        //     send_progress: SendProgress::new(0),
        //     price,
        //     transaction_context,
        // };

        // Ok(lw)

        todo!();
    }
}
