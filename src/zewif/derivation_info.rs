#[derive(Debug, Clone)]
pub struct DerivationInfo {
    change: NonHardenedChildIndex,
    address_index: NonHardenedChildIndex,
}

#[derive(Debug, Clone)]
pub struct NonHardenedChildIndex(u32);
