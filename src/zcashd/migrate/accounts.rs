use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};

use crate::{
    Account, AddressId, ProtocolAddress, TxId, u256,
    zcashd::{
        self, ZcashdWallet,
        migrate::{
            addresses::initialize_address_registry,
            keys::{convert_sapling_spending_key, find_sapling_key_for_ivk},
            tx_addresses::extract_transaction_addresses,
        },
    },
    zewif,
};

/// Convert ZCashd UnifiedAccounts to Zewif accounts
pub fn convert_unified_accounts(
    wallet: &ZcashdWallet,
    unified_accounts: &zcashd::UnifiedAccounts,
    transactions: &HashMap<TxId, zewif::Transaction>,
) -> Result<HashMap<u256, Account>> {
    let mut accounts_map = HashMap::new();

    // Step 1: Create an account for each UnifiedAccountMetadata
    for (key_id, account_metadata) in unified_accounts.account_metadata() {
        // Create a new account with the appropriate ZIP-32 account ID
        let mut account = Account::new();

        // Set the account name and ZIP-32 account ID
        let account_name = format!("Account #{}", account_metadata.account_id());
        account.set_name(account_name);
        account.set_zip32_account_id(account_metadata.account_id());

        // Store the account in our map using the key_id as the key
        accounts_map.insert(*key_id, account);
    }

    // If no accounts were created, create a default account
    if accounts_map.is_empty() {
        let mut default_account = Account::new();
        default_account.set_name("Default Account");
        accounts_map.insert(u256::default(), default_account);
    }

    // Step 2: Build an AddressRegistry to track address-to-account mappings
    let address_registry = initialize_address_registry(wallet, unified_accounts)?;

    // Step 3: Process all addresses and assign them to the appropriate accounts

    // Process transparent addresses
    for (zcashd_address, name) in wallet.address_names() {
        // Create an AddressId for this transparent address
        let addr_id = AddressId::Transparent(zcashd_address.0.clone());

        // Try to find which account this address belongs to using our registry
        let account_key_id = if let Some(key_id) = address_registry.find_account(&addr_id) {
            // Found a mapping in the registry
            *key_id
        } else {
            // No mapping found, fall back to the first account
            match accounts_map.keys().next() {
                Some(key) => *key,
                None => u256::default(),
            }
        };

        if let Some(account) = accounts_map.get_mut(&account_key_id) {
            let transparent_address = zewif::TransparentAddress::new(zcashd_address.0.clone());

            // Create a ZewifAddress from the TransparentAddress
            let protocol_address = ProtocolAddress::Transparent(transparent_address);
            let mut zewif_address = zewif::Address::new(protocol_address);
            zewif_address.set_name(name.clone());

            // Set purpose if available
            if let Some(purpose) = wallet.address_purposes().get(zcashd_address) {
                zewif_address.set_purpose(purpose.clone());
            }

            // Add the address to the account
            account.add_address(zewif_address);
        }
    }

    // Process sapling addresses
    for (sapling_address, viewing_key) in wallet.sapling_z_addresses() {
        let address_str = sapling_address.to_string(wallet.network());

        // Create an AddressId for this sapling address
        let addr_id = AddressId::Sapling(address_str.clone());

        // Try to find which account this address belongs to using our registry
        let account_key_id = if let Some(key_id) = address_registry.find_account(&addr_id) {
            // Found a mapping in the registry
            *key_id
        } else {
            // No mapping found, fall back to the first account
            match accounts_map.keys().next() {
                Some(key) => *key,
                None => u256::default(),
            }
        };

        if let Some(account) = accounts_map.get_mut(&account_key_id) {
            let address_str = sapling_address.to_string(wallet.network());

            // Create a new ShieldedAddress
            let mut shielded_address = zewif::ShieldedAddress::new(address_str.clone());
            shielded_address.set_incoming_viewing_key(viewing_key.to_owned());

            // Add spending key if available in sapling_keys
            if let Some(sapling_key) = find_sapling_key_for_ivk(wallet, viewing_key) {
                // Convert to Zewif spending key format
                let spending_key = convert_sapling_spending_key(sapling_key.key())
                    .context("Failed to convert sapling spending key")?;
                shielded_address.set_spending_key(spending_key);
            }

            let protocol_address = zewif::ProtocolAddress::Shielded(shielded_address);
            let mut zewif_address = zewif::Address::new(protocol_address);

            // Set purpose if available - convert to Address type for lookup
            let zcashd_address = zcashd::Address(address_str.clone());
            if let Some(purpose) = wallet.address_purposes().get(&zcashd_address) {
                zewif_address.set_purpose(purpose.clone());
            }

            // Add the address to the account
            account.add_address(zewif_address);
        }
    }

    // Step 4: Process viewing keys in unified_accounts
    // Each full_viewing_key entry maps a key_id to a viewing key string
    for (key_id, viewing_key) in unified_accounts.full_viewing_keys() {
        // Find the account for this key_id
        if let Some(account) = accounts_map.get_mut(key_id) {
            // TODO: Process and add the viewing key to the account
            // This will be implemented when we add specific support for viewing keys

            // For now, just log that we have a viewing key for this account
            eprintln!(
                "Found viewing key for account {}: {}",
                account.name(),
                viewing_key
            );

            // Use the registry to find all addresses associated with this account
            let account_addresses = address_registry.find_addresses_for_account(key_id);
            if !account_addresses.is_empty() {
                eprintln!("  Account has {} addresses", account_addresses.len());
            }
        }
    }

    // Step 5: Assign transactions to relevant accounts based on address involvement
    // We'll use our AddressRegistry to find account associations

    // Analyze each transaction to find which addresses are involved
    for (txid, wallet_tx) in wallet.transactions() {
        // Extract all addresses involved in this transaction as AddressId values
        match extract_transaction_addresses(wallet, *txid, wallet_tx) {
            Ok(address_ids) => {
                let mut relevant_accounts = HashSet::new();

                // Determine which accounts the transaction is relevant to by looking up
                // each extracted AddressId in the registry
                for addr_id in address_ids {
                    if let Some(account_id) = address_registry.find_account(&addr_id) {
                        relevant_accounts.insert(*account_id);
                    }
                }

                // If we couldn't determine relevant accounts, add to all accounts as fallback
                if relevant_accounts.is_empty() {
                    for account_id in accounts_map.keys() {
                        relevant_accounts.insert(*account_id);
                    }
                }

                // Add the transaction to relevant accounts
                for account_id in relevant_accounts {
                    if let Some(account) = accounts_map.get_mut(&account_id) {
                        account.add_relevant_transaction(*txid);
                    }
                }
            }
            Err(e) => {
                // Log the error and fall back to adding the transaction to all accounts
                eprintln!("Error analyzing transaction {}: {}", txid, e);
                for account in accounts_map.values_mut() {
                    account.add_relevant_transaction(*txid);
                }
            }
        }
    }

    // Step 6: Perform validation to ensure all transactions have appropriate associations
    let total_tx_count = transactions.len();
    let mut total_account_tx_count = 0;
    let mut accounts_with_txs = 0;

    for account in accounts_map.values() {
        let account_tx_count = account.relevant_transactions().len();
        total_account_tx_count += account_tx_count;

        if account_tx_count > 0 {
            accounts_with_txs += 1;
        }
    }

    // Log statistics to help verify transaction assignment
    eprintln!("Transaction assignment complete:");
    eprintln!("Total transactions: {}", total_tx_count);
    eprintln!("Total account transactions: {}", total_account_tx_count);
    eprintln!(
        "Accounts with transactions: {}/{}",
        accounts_with_txs,
        accounts_map.len()
    );

    // If there are no account assignments at all, something might be wrong
    if total_account_tx_count == 0 && total_tx_count > 0 {
        eprintln!("Warning: No transactions were assigned to any accounts!");
    }

    Ok(accounts_map)
}
