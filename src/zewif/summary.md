```rust
#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

use bc_components::{Digest, ARID};
use bc_envelope::prelude::*;

use crate::{Amount, Blob, BlockHeight, Data, TxId};

/// A trait for objects that have a unique identifier within the wallet
/// interchange format.
///
/// QUESTION: Should all possible wallet objects implement this trait, or only
/// some? What criteria should be used to determine this?
pub trait Identifiable {
    fn id(&self) -> &ARID;
}

/// Represents a wallet database, the top level of the interchange format
/// hierarchy, which can contain multiple wallets and a global transaction
/// history.
#[derive(Debug, Clone)]
pub struct WalletDB {
    wallets: HashMap<ARID, Wallet>,
    transactions: HashMap<TxId, Transaction>,
    attachments: Attachments,
}

/// Represents an entire wallet, including multiple accounts, a wallet-specific
/// subset of the global transaction history, and optionally a form of seed
/// material for generating cryptographic keys.
#[derive(Debug, Clone)]
pub struct Wallet {
    id: ARID,
    seed_material: Option<SeedMaterial>,
    accounts: HashMap<ARID, Account>,
    attachments: Attachments,
}

/// Further impls of this omitted for brevity.
impl Identifiable for Wallet {
    fn id(&self) -> &ARID {
        &self.id
    }
}

/// Seed material used to generate the keys in the wallet.
/// Proposal as minimal set of sources of truth
#[derive(Debug, Clone)]
pub enum SeedMaterial {
    Bip39Mnemonic(String),
    PreBIP39Seed(Blob<32>),
}

/// Logical grouping within a wallet. Each account can have its own set of
/// addresses, transactions, and other metadata.
#[derive(Debug, Clone)]
pub struct Account {
    id: ARID,
    name: String, // May not be unique.
    zip32_account_id: Option<u32>,
    addresses: HashMap<String, Address>,
    relevant_transactions: HashSet<TxId>, // Subset of the global transaction history.
    // The following are intended for storage of information that may not be
    // recoverable from the chain.
    sapling_sent_outputs: Vec<SaplingSentOutput>,
    orchard_sent_outputs: Vec<OrchardSentOutput>,
    attachments: Attachments,
}

#[derive(Debug, Clone)]
pub struct SaplingSentOutput; // QUESTION: Needs definition

#[derive(Debug, Clone)]
pub struct OrchardSentOutput; // QUESTION: Needs definition

/// A wallet address can be either an exposed transparent address or one of several shielded types.
#[derive(Debug, Clone)]
pub enum Address {
    /// An exposed transparent (T-address) similar to Bitcoin's.
    Transparent(TransparentAddress),
    /// A shielded address (Z-address). This can include Sapling, Sprout, or Orchard formats.
    Shielded(ShieldedAddress),
}

#[derive(Debug, Clone)]
pub struct TransparentAddress {
    address: String, // Unique
    spend_authority: Option<TransparentSpendAuthority>,
    derivation_info: Option<DerivationInfo>,
    attachments: Attachments,
}

/// Details specific to shielded addresses.
#[derive(Debug, Clone)]
pub struct ShieldedAddress {
    /// The actual address string (could encode Sapling, Orchard, etc.).
    address: String, // Unique
    /// Optional diversifier or other Zcash-specific metadata.
    diversifier: Option<Data>,
    attachments: Attachments,
}

/// The authority to spend from a transparent address.
#[derive(Debug, Clone)]
pub enum TransparentSpendAuthority {
    SpendingKey(SpendingKey),
    Derived,
}

#[derive(Debug, Clone)]
pub struct SpendingKey(Blob<32>);

#[derive(Debug, Clone)]
pub struct DerivationInfo {
    change: NonHardenedChildIndex,
    address_index: NonHardenedChildIndex,
}

#[derive(Debug, Clone)]
pub struct NonHardenedChildIndex(u32);

/// A transaction that can combine both transparent and shielded components.
#[derive(Debug, Clone)]
pub struct Transaction {
    /// The transaction id.
    txid: TxId,
    /// The raw transaction data, if known.
    raw: Option<Data>,
    /// The height at which the transaction was mined, if known.
    /// It is possible that if a rollback occurred just after the zeWIF
    /// export, the transaction could have been unmined, and possibly
    /// remined at a different height.
    mined_height: Option<BlockHeight>,

    // Design issue: do we want to parse out all of this? All wallets will
    // necessarily have code to parse a transaction. The only information
    // that is not redundant with the raw transaction encoding is the
    // *decrypted* note plaintexts (and it might be sufficient to just
    // indicate which output indices are expected to be decryptable with
    // which keys). I don't see the point of duplicating the raw data in a
    // different format (that still needs to be parsed!)
    // -- Daira-Emma

    /// Optional data for transparent inputs
    inputs: Option<Vec<TxIn>>,
    /// Optional data for transparent outputs
    outputs: Option<Vec<TxOut>>,
    /// Optional data for Sapling spends
    sapling_spends: Option<Vec<SaplingSpendDescription>>,
    /// Optional data for Sapling outputs
    sapling_outputs: Option<Vec<SaplingOutputDescription>>,
    /// Optional data for Orchard actions
    orchard_actions: Option<Vec<OrchardActionDescription>>,
    /// Optional data for Sprout JoinSplit descriptions
    sprout_joinsplits: Option<Vec<JoinSplitDescription>>,
    // Additional metadata such as confirmations or timestamp may be added here.
    attachments: Attachments,
}

/// A reference to a previous transaction output.
#[derive(Debug, Clone)]
pub struct TxOutPoint {
    txid: TxId,
    index: u32,
}

/// A transparent transaction input.
#[derive(Debug, Clone)]
pub struct TxIn {
    previous_output: TxOutPoint,
    /// Script signature for unlocking the previous output.
    script_sig: Data,
    sequence: u32,
}

/// A transparent transaction output.
#[derive(Debug, Clone)]
pub struct TxOut {
    value: Amount,
    script_pubkey: Data,
}

/// Data specific to Sapling spends.
#[derive(Debug, Clone)]
pub struct SaplingSpendDescription {
    spend_index: u32,
    /// The value of the input note, if known.
    value: Option<Amount>,
    /// The height that the anchor corresponds to, if known.
    anchor_height: Option<BlockHeight>,
    /// A nullifier to ensure the note is spent only once.
    nullifier: Blob<32>,
    /// A zero-knowledge proof that the spend is valid.
    zkproof: Data,
    // Additional fields (e.g., spending key components) may be required.
    attachments: Attachments,
}

/// Data specific to Sapling outputs.
/// *this is currently not sufficient to make all outputs recoverable*
/// Additional things that are needed:
/// * recipient address (unified address that was specified as the recipient by the user)
/// * value of the output
/// * other information in https://github.com/zcash/librustzcash/blob/54fd075449218e27e4cbf5cb7108cd89f458acc0/zcash_client_sqlite/src/wallet/db.rs#L455 may not be recoverable from the chain
#[derive(Debug, Clone)]
pub struct SaplingOutputDescription {
    output_index: u32,
    /// The note commitment.
    commitment: Blob<32>,
    /// Ephemeral key for the encrypted note.
    ephemeral_key: Blob<32>,
    /// Encrypted ciphertext containing the note details.
    enc_ciphertext: Data,
    /// An optional memo field.
    memo: Option<Data>,
    /// This and the witness are recorded at export as of
    /// an anchor depth 20 blocks back from the chain tip, or the oldest possible witness at a lesser depth.
    note_commitment_tree_position: Position,
    /// Witness
    witness: Option<(Anchor, IncrementalWitness)>,
    attachments: Attachments,
}

/// Data specific to Orchard actions.
/// TODO CHECK
#[derive(Debug, Clone)]
pub struct OrchardActionDescription {
    action_index: u32,
    /// The anchor of the current commitment tree.
    anchor: Blob<32>,
    /// A nullifier to ensure the note is spent only once.
    nullifier: Blob<32>,
    /// A zero-knowledge proof that the spend is valid.
    zkproof: Data,
    /// Additional fields (e.g., spending key components) may be required.
    /// The note commitment.
    commitment: Blob<32>,
    /// Ephemeral key for the encrypted note.
    ephemeral_key: Blob<32>,
    /// Encrypted ciphertext containing the note details.
    enc_ciphertext: Data,
    /// An optional memo field.
    memo: Option<Data>,
    /// This and the witness are recorded at export as of
    /// an anchor depth 20 blocks back from the chain tip, or the oldest possible witness at a lesser depth.
    note_commitment_tree_position: Position,
    /// Witness
    witness: Option<(Anchor, IncrementalWitness)>,
    attachments: Attachments,
}

/// For legacy Sprout transactions: JoinSplit descriptions that mix transparent and shielded values.
#[derive(Debug, Clone)]
pub struct JoinSplitDescription {
    anchor: Anchor,
    nullifiers: [Blob<32>; 2],
    commitments: [Blob<32>; 2],
    /// A zero-knowledge proof to validate the JoinSplit operation.
    zkproof: Data,
    // Further fields may be added as necessary.
    attachments: Attachments,
}

/// A position in a note commitment tree.
#[derive(Debug, Clone)]
pub struct Position(u32);

#[derive(Debug, Clone)]
pub struct Anchor(Blob<32>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalWitness<const DEPTH: usize, Hash> {
    pub tree: IncrementalMerkleTree,
    pub filled: Vec<Hash>,
    pub cursor: Option<IncrementalMerkleTree>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IncrementalMerkleTree {
    pub left: Option<u256>,
    pub right: Option<u256>,
    pub parents: Vec<Option<u256>>,
}
```
