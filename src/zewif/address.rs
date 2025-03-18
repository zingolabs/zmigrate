use super::{ShieldedAddress, TransparentAddress};

/// A wallet address can be either an exposed transparent address or one of several shielded types.
#[derive(Debug, Clone)]
pub enum Address {
    /// An exposed transparent (T-address) similar to Bitcoin's.
    Transparent(TransparentAddress),
    /// A shielded address (Z-address). This can include Sapling, Sprout, or Orchard formats.
    Shielded(ShieldedAddress),
}
