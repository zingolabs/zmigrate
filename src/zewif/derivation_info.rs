#[derive(Debug, Clone)]
pub struct DerivationInfo {
    pub change: NonHardenedChildIndex,
    pub address_index: NonHardenedChildIndex,
}

#[derive(Debug, Clone)]
pub struct NonHardenedChildIndex(pub u32);
