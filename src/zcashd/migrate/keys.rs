use anyhow::Result;

use crate::{zcashd::{self, ZcashdWallet}, zewif, SaplingIncomingViewingKey};

/// Find a SaplingKey for a given incoming viewing key
pub fn find_sapling_key_for_ivk<'a>(
    wallet: &'a ZcashdWallet,
    ivk: &SaplingIncomingViewingKey,
) -> Option<&'a zcashd::SaplingKey> {
    wallet.sapling_keys().get(ivk)
}

/// Convert ZCashd SaplingExtendedSpendingKey to Zewif SpendingKey
pub fn convert_sapling_spending_key(
    key: &crate::sapling::SaplingExtendedSpendingKey,
) -> Result<zewif::SpendingKey> {
    // Create the Sapling spending key with all components including HD parameters
    // Since both structures use u256, we can directly use them without cloning
    let spending_key = zewif::SpendingKey::new_sapling_extended(
        key.expsk.ask,
        key.expsk.nsk,
        key.expsk.ovk,
        key.depth,
        key.parent_fvk_tag,
        key.child_index,
        key.chain_code,
        key.dk,
    );

    Ok(spending_key)
}
