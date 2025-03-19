use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ProtocolAddress, SaplingIncomingViewingKey, TxId,
    zcashd::{self, ZcashdWallet},
    zewif::{self, ZewifTop, ZewifWallet},
};

/// Migrate a ZCashd wallet to the Zewif wallet format
pub fn migrate_to_zewif(wallet: &ZcashdWallet) -> Result<ZewifTop> {
    // Create a new ZewifDB
    let mut zewif_top = ZewifTop::new();

    // Convert seed material (mnemonic phrase)
    let seed_material = convert_seed_material(wallet)?;

    // Create a default account to hold all addresses
    let mut account = zewif::Account::new();

    // Convert transparent addresses
    convert_transparent_addresses(wallet, &mut account)?;

    // Convert sapling addresses
    convert_sapling_addresses(wallet, &mut account)?;

    // Process transactions and collect relevant transaction IDs
    let transactions = convert_transactions(wallet)?;

    // Add all transaction IDs to the account's relevant transactions
    for txid in transactions.keys() {
        account.add_relevant_transaction(*txid);
    }

    // Create a complete Zewif wallet
    let mut zewif_wallet = ZewifWallet::new(wallet.network());

    if let Some(seed_material) = seed_material {
        zewif_wallet.set_seed_material(seed_material);
    }

    zewif_wallet.add_account(account);

    // Add wallet and transactions to the ZewifDB
    zewif_top.add_wallet(zewif_wallet);
    zewif_top.transactions = transactions;

    Ok(zewif_top)
}

/// Convert ZCashd mnemonic seed to Zewif SeedMaterial
fn convert_seed_material(wallet: &ZcashdWallet) -> Result<Option<zewif::SeedMaterial>> {
    // Check if we have a mnemonic phrase
    if !wallet.bip39_mnemonic.mnemonic().is_empty() {
        return Ok(Some(zewif::SeedMaterial::Bip39Mnemonic(
            wallet.bip39_mnemonic.mnemonic().clone(),
        )));
    }
    // If no mnemonic, return None
    Ok(None)
}

/// Convert ZCashd transparent addresses to Zewif format
fn convert_transparent_addresses(
    wallet: &ZcashdWallet,
    account: &mut zewif::Account,
) -> Result<()> {
    // Process address_names which contain transparent addresses
    for (zcashd_address, name) in &wallet.address_names {
        let transparent_address = zewif::TransparentAddress::new(zcashd_address.0.clone());

        // Create a ZewifAddress from the TransparentAddress
        let protocol_address = ProtocolAddress::Transparent(transparent_address);
        let mut zewif_address = zewif::Address::new(protocol_address);
        zewif_address.set_name(name.clone());

        // Set purpose if available
        if let Some(purpose) = wallet.address_purposes.get(zcashd_address) {
            zewif_address.set_purpose(purpose.clone());
        }

        // Add the address to the account with its string representation as key
        account.add_address(zewif_address);
    }

    Ok(())
}

/// Convert ZCashd sapling addresses to Zewif format
fn convert_sapling_addresses(wallet: &ZcashdWallet, account: &mut zewif::Account) -> Result<()> {
    // Process sapling_z_addresses
    for (sapling_address, viewing_key) in &wallet.sapling_z_addresses {
        let address_str = sapling_address.to_string(wallet.network());

        // Create a new ShieldedAddress
        let mut shielded_address = zewif::ShieldedAddress::new(address_str.clone());
        shielded_address.set_incoming_viewing_key(viewing_key.clone());

        // Add spending key if available in sapling_keys
        if let Some(sapling_key) = find_sapling_key_for_ivk(wallet, viewing_key) {
            // Convert to Zewif spending key format
            let spending_key = convert_sapling_spending_key(&sapling_key.key)?;
            shielded_address.set_spending_key(spending_key);
        }

        let protocol_address = zewif::ProtocolAddress::Shielded(shielded_address);
        let mut zewif_address = zewif::Address::new(protocol_address);

        // Set purpose if available - convert to Address type for lookup
        let zcashd_address = zcashd::Address(address_str.clone());
        if let Some(purpose) = wallet.address_purposes.get(&zcashd_address) {
            zewif_address.set_purpose(purpose.clone());
        }

        // Add the address to the account with its string representation as key
        account.add_address(zewif_address);
    }

    Ok(())
}

/// Find a SaplingKey for a given incoming viewing key
fn find_sapling_key_for_ivk<'a>(
    wallet: &'a ZcashdWallet,
    ivk: &SaplingIncomingViewingKey,
) -> Option<&'a zcashd::SaplingKey> {
    wallet.sapling_keys.0.get(ivk)
}

/// Convert ZCashd SaplingExtendedSpendingKey to Zewif SpendingKey
fn convert_sapling_spending_key(
    key: &crate::sapling::SaplingExtendedSpendingKey,
) -> Result<zewif::SpendingKey> {
    // For now, we'll create a simplified representation of the key
    // In a real implementation, we would use proper ZCash serialization

    // We'll use the expanded spending key's ask value as the primary component
    // of our simplified spending key representation
    let ask_bytes = key.expsk.ask.as_ref();

    // Create a blob with the 32-byte array from the ask field
    let mut key_bytes = [0u8; 32];
    key_bytes.copy_from_slice(ask_bytes);

    let blob = zewif::Blob::new(key_bytes);

    Ok(zewif::SpendingKey(blob))
}

/// Convert ZCashd transactions to Zewif format
fn convert_transactions(wallet: &ZcashdWallet) -> Result<HashMap<TxId, zewif::Transaction>> {
    let mut transactions = HashMap::new();

    for (tx_id, wallet_tx) in &wallet.transactions {
        let zewif_tx = convert_transaction(*tx_id, wallet_tx)?;
        transactions.insert(*tx_id, zewif_tx);
    }

    Ok(transactions)
}

/// Convert a single ZCashd transaction to Zewif format
fn convert_transaction(tx_id: TxId, tx: &zcashd::WalletTx) -> Result<zewif::Transaction> {
    let zewif_tx = zewif::Transaction::new(tx_id);

    // TODO: Convert transaction details

    Ok(zewif_tx)
}

impl From<&ZcashdWallet> for Result<ZewifTop> {
    fn from(wallet: &ZcashdWallet) -> Self {
        migrate_to_zewif(wallet)
    }
}
