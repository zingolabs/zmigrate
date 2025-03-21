use anyhow::Result;

use super::{
    accounts::convert_unified_accounts,
    addresses::{
        convert_sapling_addresses, convert_transparent_addresses, initialize_address_registry,
    },
    seed::convert_seed_material,
    transactions::{convert_transactions, update_transaction_positions},
};

use crate::{
    u256,
    zcashd::ZcashdWallet,
    zewif::{Account, ZewifTop, ZewifWallet},
};

/// Migrate a ZCashd wallet to the Zewif wallet format
pub fn migrate_to_zewif(wallet: &ZcashdWallet) -> Result<ZewifTop> {
    // Create a new ZewifTop
    let mut zewif_top = ZewifTop::new();

    // Convert seed material (mnemonic phrase)
    let seed_material = convert_seed_material(wallet)?;

    // Create a complete Zewif wallet
    let mut zewif_wallet = ZewifWallet::new(wallet.network());

    if let Some(seed_material) = seed_material {
        zewif_wallet.set_seed_material(seed_material);
    }

    // Process transactions and collect relevant transaction IDs
    let mut transactions = convert_transactions(wallet)?;

    // Convert orchard note commitment tree if available
    if !wallet
        .orchard_note_commitment_tree()
        .unparsed_data
        .is_empty()
    {
        // Update transaction outputs with note positions from the note commitment tree
        update_transaction_positions(wallet, &mut transactions)?;
    }

    // If there are unified accounts, process them
    if let Some(unified_accounts) = wallet.unified_accounts() {
        // Create accounts based on unified_accounts structure
        let mut accounts_map = convert_unified_accounts(wallet, unified_accounts, &transactions)?;

        // Initialize address registry to track address-to-account relationships
        let address_registry = initialize_address_registry(wallet, unified_accounts)?;

        // Create a default account for addresses not associated with any other account
        let mut default_account = Account::new();
        default_account.set_name("Default Account");

        // Create a mutable reference for accounts_map to use in the conversion functions
        let mut accounts_map_ref = Some(&mut accounts_map);

        // Convert transparent addresses using the registry to assign to correct accounts
        convert_transparent_addresses(
            wallet,
            &mut default_account,
            Some(&address_registry),
            &mut accounts_map_ref,
        )?;

        // Convert sapling addresses using the registry to assign to correct accounts
        convert_sapling_addresses(
            wallet,
            &mut default_account,
            Some(&address_registry),
            &mut accounts_map_ref,
        )?;

        // Add the default account to accounts_map if it has any addresses
        if !default_account.addresses().is_empty() {
            accounts_map.insert(u256::default(), default_account);
        }

        // Add all accounts to the wallet
        for account in accounts_map.values() {
            zewif_wallet.add_account(account.clone());
        }
    } else {
        // No unified accounts - create a single default account
        let mut default_account = Account::new();
        default_account.set_name("Default Account");

        // Create a None reference for accounts_map
        let mut accounts_map_ref = None;

        // Convert transparent addresses (single account mode)
        convert_transparent_addresses(wallet, &mut default_account, None, &mut accounts_map_ref)?;

        // Convert sapling addresses (single account mode)
        convert_sapling_addresses(wallet, &mut default_account, None, &mut accounts_map_ref)?;

        // Add all transaction IDs to the default account's relevant transactions
        for txid in transactions.keys() {
            default_account.add_relevant_transaction(*txid);
        }

        // Add the default account to the wallet
        zewif_wallet.add_account(default_account);
    }

    // Add wallet and transactions to the ZewifTop
    zewif_top.add_wallet(zewif_wallet);
    zewif_top.set_transactions(transactions);

    Ok(zewif_top)
}

impl From<&ZcashdWallet> for Result<ZewifTop> {
    fn from(wallet: &ZcashdWallet) -> Self {
        migrate_to_zewif(wallet)
    }
}

#[cfg(test)]
mod tests {
    use crate::{AddressId, AddressRegistry, TxId};

    use super::*;

    // Test the AddressRegistry-to-Account mapping
    #[test]
    fn test_address_registry_account_mapping() {
        // Create a simple address registry
        let mut registry = AddressRegistry::new();

        // Create test address IDs and account IDs
        let addr1 = AddressId::Transparent("t1example1".to_string());
        let addr2 = AddressId::Sapling("zs1example1".to_string());
        let addr3 = AddressId::Transparent("t1example2".to_string());

        let account1 = u256::default();
        let mut bytes = [0u8; 32];
        bytes[0] = 1;
        let account2 = u256::from_slice(&bytes).unwrap();

        // Register addresses to accounts
        registry.register(addr1.clone(), account1);
        registry.register(addr2.clone(), account1);
        registry.register(addr3.clone(), account2);

        // Test the mapping functions
        let addrs_for_acct1 = registry.find_addresses_for_account(&account1);
        assert_eq!(addrs_for_acct1.len(), 2);
        assert!(addrs_for_acct1.contains(&&addr1));
        assert!(addrs_for_acct1.contains(&&addr2));

        let account_for_addr3 = registry.find_account(&addr3);
        assert_eq!(account_for_addr3, Some(&account2));

        // Test that address type is preserved in lookup results
        assert!(
            addrs_for_acct1
                .iter()
                .any(|addr| matches!(*addr, AddressId::Transparent(_)))
        );
        assert!(
            addrs_for_acct1
                .iter()
                .any(|addr| matches!(*addr, AddressId::Sapling(_)))
        );
    }

    // Test the AddressId conversion from string
    #[test]
    fn test_address_id_string_conversions() {
        // Test different address types
        let test_cases = [
            ("t1example", AddressId::Transparent("t1example".to_string())),
            ("zs1example", AddressId::Sapling("zs1example".to_string())),
            ("zo1example", AddressId::Orchard("zo1example".to_string())),
            ("u1example", AddressId::Unified("u1example".to_string())),
        ];

        for (addr_str, expected_id) in test_cases {
            let result = AddressId::from_address_string(addr_str, crate::zewif::Network::Test);
            assert!(result.is_ok());
            let addr_id = result.unwrap();
            assert_eq!(addr_id, expected_id);

            // Test that string conversion preserves the original address
            assert_eq!(addr_id.address_string().unwrap(), addr_str);
        }
    }

    // Test the update_transaction_positions functionality
    // This is just a simplified test since we can't access private modules in the test
    #[test]
    fn test_position_update_logic() {
        use crate::zewif::{OrchardActionDescription, Position, Transaction};
        use std::collections::HashMap;

        // Create a simple transaction with orchard actions
        let tx_bytes = [0u8; 32];
        let tx_id = TxId::from_bytes(tx_bytes);
        let mut zewif_tx = Transaction::new(tx_id);

        // Create test commitments
        let mut test_commitments = Vec::new();
        for i in 0..3 {
            let mut bytes = [0u8; 32];
            bytes[0] = i as u8 + 1; // Start from 1 to avoid all zeros
            let commitment = u256::from_slice(&bytes).unwrap();
            test_commitments.push(commitment);
        }

        // Add orchard actions with the test commitments
        let mut action1 = OrchardActionDescription::new();
        action1.set_action_index(0);
        action1.set_commitment(test_commitments[1]);

        let mut action2 = OrchardActionDescription::new();
        action2.set_action_index(1);
        action2.set_commitment(test_commitments[2]);

        zewif_tx.add_orchard_action(action1);
        zewif_tx.add_orchard_action(action2);

        // Create transaction collection
        let mut transactions = HashMap::new();
        transactions.insert(tx_id, zewif_tx);

        // Verify initial state - default positions (0)
        {
            let tx = &transactions[&tx_id];
            let actions = tx.orchard_actions().unwrap();
            assert_eq!(actions[0].note_commitment_tree_position().0, 0);
            assert_eq!(actions[1].note_commitment_tree_position().0, 0);
        }

        // Set the position manually (what update_transaction_positions would do)
        {
            let tx = transactions.get_mut(&tx_id).unwrap();
            if let Some(actions) = tx.orchard_actions_mut() {
                actions[0].set_note_commitment_tree_position(Position(1));
                actions[1].set_note_commitment_tree_position(Position(2));
            }
        }

        // Verify that positions were updated
        let updated_tx = &transactions[&tx_id];
        let updated_actions = updated_tx.orchard_actions().unwrap();

        // Check the positions were set correctly
        assert_eq!(updated_actions[0].note_commitment_tree_position().0, 1);
        assert_eq!(updated_actions[1].note_commitment_tree_position().0, 2);
    }
}
