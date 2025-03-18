use crate::{Amount, Blob};

/// Represents a sent output in an Orchard shielded transaction within a Zcash wallet.
///
/// This structure stores the plaintext details of an Orchard note that was sent by the
/// wallet, which are not recoverable from the blockchain. It supports selective disclosure
/// for proving payments to shielded addresses, enhancing privacy and compliance features.
/// Introduced in Network Upgrade 5 (NU5), the Orchard protocol builds on Sapling with
/// additional privacy enhancements, using the Pallas curve and new cryptographic primitives.
#[derive(Debug, Clone)]
pub struct OrchardSentOutput {
    /// The diversifier used in deriving the recipient's shielded address.
    ///
    /// This 11-byte value serves the same purpose as in Sapling, enabling address
    /// diversity for privacy. It is part of the note plaintext and essential for
    /// identifying the recipient during selective disclosure.
    pub diversifier: Blob<11>,

    /// The recipient's public key, serialized in compressed form.
    ///
    /// This 32-byte value represents a point on the Pallas curve, distinct from Sapling's
    /// Jubjub curve. It is included in the note plaintext and necessary for verifying
    /// the recipient in proofs or audits.
    pub receipient_public_key: Blob<32>,

    /// The value of ZEC sent in this output, in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// This 64-bit unsigned integer denotes the amount sent, with the same maximum value
    /// constraint as Sapling (2^63 - 1 zatoshis). It is a core component of the note
    /// for tracking and proving the transaction amount.
    pub value: Amount,

    /// A randomness element used in Orchard's note encryption and commitment.
    ///
    /// This 32-byte value (an element of the Pallas curve's field F_q) is unique to Orchard,
    /// enhancing privacy by contributing to the note's uniqueness. It is stored for
    /// reconstructing the note during selective disclosure.
    pub rho: Blob<32>,

    /// Another randomness element used in Orchard's note construction.
    ///
    /// This 32-byte value (also an element of F_q) further strengthens privacy in Orchard
    /// transactions. It is part of the note plaintext and required for generating proofs
    /// that validate the sent output.
    pub psi: Blob<32>,

    /// The random commitment material used in the note commitment.
    ///
    /// This 32-byte value (256-bit scalar) serves a similar role to Sapling's rcm, masking
    /// the note's contents on the blockchain. It is stored to enable the wallet to
    /// regenerate the commitment for proving payment details.
    pub rcm: Blob<32>,
}
