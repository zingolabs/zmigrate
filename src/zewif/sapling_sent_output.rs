use crate::{Amount, Blob};

/// Represents a sent output in a Sapling shielded transaction within a Zcash wallet.
///
/// This structure stores the plaintext details of a Sapling note that was sent by the wallet,
/// which are not recoverable from the blockchain. It is used for selective disclosure,
/// allowing the sender to prove a payment was made to a specific shielded address without
/// revealing the full transaction details. This is particularly useful for auditing or
/// compliance with regulations. The Sapling protocol, activated in June 2018, enhances
/// privacy using zk-SNARKs over the earlier Sprout protocol.
#[derive(Debug, Clone)]
pub struct SaplingSentOutput {
    /// The diversifier used in deriving the recipient's shielded address.
    ///
    /// This 11-byte value is part of the Sapling address construction, allowing multiple
    /// unique addresses to be generated from a single key pair. It is critical for
    /// identifying the recipient and reconstructing the note for proof generation.
    pub diversifier: Blob<11>,

    /// The recipient's public key, serialized in compressed form.
    ///
    /// This 32-byte value represents a point on the Jubjub curve, used in Sapling's
    /// cryptographic operations. It is part of the note plaintext and is needed to
    /// verify the recipient of the sent funds during selective disclosure.
    pub receipient_public_key: Blob<32>,

    /// The value of ZEC sent in this output, in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// This 64-bit unsigned integer specifies the amount transferred. It is constrained
    /// by the protocol to a maximum value (2^63 - 1 zatoshis), ensuring it fits within
    /// the note's value field for Sapling transactions.
    pub value: Amount,

    /// The random commitment material used in the note commitment.
    ///
    /// This 32-byte value (256-bit scalar) is a randomly generated element used to
    /// construct the note commitment on the blockchain, ensuring privacy by masking
    /// the note's contents. It is stored here to allow reconstruction of the commitment
    /// for proving purposes.
    pub rcm: Blob<32>,
}
