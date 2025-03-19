use std::collections::HashMap;

use anyhow::Result;

use crate::{
    ProtocolAddress, TxId,
    zcashd::{self, ZcashdWallet},
    zewif::{self, ZewifTop, ZewifWallet},
};

/// Migrate a ZCashd wallet to the Zewif wallet format
pub fn migrate_to_zewif(wallet: &ZcashdWallet) -> Result<ZewifTop> {
    // Create a new ZewifDB
    let mut zewif_db = ZewifTop::new();

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
    let mut zewif_wallet = ZewifWallet::new();
    zewif_wallet.seed_material = seed_material;

    // Add wallet and transactions to the ZewifDB
    zewif_db.add_wallet(zewif_wallet);
    zewif_db.transactions = transactions;

    Ok(zewif_db)
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
        let protocol_address = zewif::ProtocolAddress::Shielded(shielded_address);
        let zewif_address = zewif::Address::new(protocol_address);

        // Add the address to the account with its string representation as key
        account.add_address(zewif_address);
    }

    Ok(())
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

    Ok(zewif_tx)
}

impl From<&ZcashdWallet> for Result<ZewifTop> {
    fn from(wallet: &ZcashdWallet) -> Self {
        migrate_to_zewif(wallet)
    }
}
