#[derive(Debug, Clone, Copy)]
pub struct DerivationInfo {
    change: NonHardenedChildIndex,
    address_index: NonHardenedChildIndex,
}

impl DerivationInfo {
    pub fn change(&self) -> NonHardenedChildIndex {
        self.change
    }

    pub fn address_index(&self) -> NonHardenedChildIndex {
        self.address_index
    }
}

#[derive(Debug, Clone, Copy)]
pub struct NonHardenedChildIndex(pub u32);
