use std::collections::HashSet;

use anyhow::Result;
use ripemd::{Digest, Ripemd160};
use sha2::Sha256;

use crate::{zcashd::{self, ZcashdWallet}, AddressId, TxId};

/// Extract all addresses involved in a transaction and return as AddressId values
pub fn extract_transaction_addresses(
    wallet: &ZcashdWallet,
    tx_id: TxId,
    tx: &zcashd::WalletTx,
) -> Result<HashSet<AddressId>> {
    let mut addresses: HashSet<AddressId> = HashSet::new();
    // Temporary storage for string addresses (used for metadata addresses that can't be converted to AddressId)
    let mut meta_addresses: HashSet<String> = HashSet::new();

    // Check if we have recipient mappings for this transaction
    if let Some(recipients) = wallet.send_recipients().get(&tx_id) {
        for recipient in recipients {
            // Add the unified address if it exists
            if !recipient.unified_address().is_empty() {
                if let Ok(addr_id) =
                    AddressId::from_address_string(recipient.unified_address(), wallet.network())
                {
                    addresses.insert(addr_id);
                }
            }

            // Add the recipient address based on the type
            match recipient.recipient_address() {
                zcashd::RecipientAddress::Sapling(addr) => {
                    let addr_str = addr.to_string(wallet.network());
                    addresses.insert(AddressId::Sapling(addr_str));
                }
                zcashd::RecipientAddress::Orchard(addr) => {
                    let addr_str = addr.to_string(wallet.network());
                    addresses.insert(AddressId::Orchard(addr_str));
                }
                zcashd::RecipientAddress::KeyId(key_id) => {
                    // Convert P2PKH key hash to a Zcash address
                    let addr_str = key_id.to_string(wallet.network());
                    addresses.insert(AddressId::Transparent(addr_str));
                }
                zcashd::RecipientAddress::ScriptId(script_id) => {
                    // Convert P2SH script hash to a Zcash address
                    let addr_str = script_id.to_string(wallet.network());
                    addresses.insert(AddressId::Transparent(addr_str));
                }
            }
        }
    }

    // For transparent inputs, extract addresses from the script signatures
    for tx_in in &tx.vin {
        // We'll derive a unique identifier from the previous outpoint to ensure we capture this transaction
        let txid_str = format!("{}", tx_in.prevout().txid());
        let input_addr = format!("input:{}:{}", txid_str, tx_in.prevout().vout());
        meta_addresses.insert(input_addr);

        // Extract potential P2PKH or P2SH addresses from script signatures
        let script_data = &tx_in.script_sig();

        // We're looking for common script signature patterns that might contain addresses
        // P2PKH scriptSigs typically have format: <sig> <pubkey>
        // P2SH scriptSigs typically have format: <...> <redeemscript>

        // For P2PKH signatures, the pubkey is near the end and we can extract it
        if script_data.len() > 33 {
            // Minimum size for a compressed pubkey (33 bytes)
            let potential_pubkey = &script_data[script_data.len() - 33..];

            // Check if it could be a compressed pubkey (starts with 0x02 or 0x03)
            if potential_pubkey[0] == 0x02 || potential_pubkey[0] == 0x03 {
                // Hash the pubkey to get the pubkey hash (RIPEMD160(SHA256(pubkey)))
                // First calculate SHA256 hash
                let mut sha256 = Sha256::new();
                sha256.update(potential_pubkey);
                let sha256_result = sha256.finalize();

                // Calculate RIPEMD160 hash of the SHA256 result
                let mut ripemd160 = Ripemd160::new();
                ripemd160.update(sha256_result);
                let pubkey_hash = ripemd160.finalize();

                // Create a transparent P2PKH address from this pubkey hash
                // Create a KeyId for consistent address encoding
                let key_id = zcashd::KeyId(
                    crate::u160::from_slice(&pubkey_hash[..])
                        .expect("Creating u160 from RIPEMD160 hash"),
                );
                let addr_str = key_id.to_string(wallet.network());
                addresses.insert(AddressId::Transparent(addr_str));
            }
        }
    }

    // For transparent outputs, extract addresses from the scriptPubKey
    for (vout_idx, tx_out) in tx.vout.iter().enumerate() {
        let script_data = tx_out.script_pub_key();

        // P2PKH detection - match the pattern: OP_DUP OP_HASH160 <pubKeyHash> OP_EQUALVERIFY OP_CHECKSIG
        if script_data.len() >= 25 && script_data[0] == 0x76 && script_data[1] == 0xA9 {
            if script_data.len() >= 25 && script_data[23] == 0x88 && script_data[24] == 0xAC {
                // The pubkey hash is 20 bytes starting at offset 3
                let pubkey_hash = &script_data[3..23];

                // Convert to a proper P2PKH Zcash address using KeyId
                let key_id = zcashd::KeyId(
                    crate::u160::from_slice(pubkey_hash).expect("Creating u160 from pubkey hash"),
                );
                let addr_str = key_id.to_string(wallet.network());
                addresses.insert(AddressId::Transparent(addr_str));
            }
        }
        // P2SH detection - match the pattern: OP_HASH160 <scriptHash> OP_EQUAL
        else if script_data.len() >= 23
            && script_data[0] == 0xA9
            && script_data.len() >= 23
            && script_data[22] == 0x87
        {
            // The script hash is 20 bytes starting at offset 2
            let script_hash = &script_data[2..22];

            // Convert to a proper P2SH Zcash address using ScriptId
            let script_id = zcashd::ScriptId(
                crate::u160::from_slice(script_hash).expect("Creating u160 from script hash"),
            );
            let addr_str = script_id.to_string(wallet.network());
            addresses.insert(AddressId::Transparent(addr_str));
        }

        // Always add an output identifier that links to this transaction
        let output_addr = format!("output:{}:{}", tx_id, vout_idx);
        meta_addresses.insert(output_addr);
    }

    // For Sapling spends and outputs
    match &tx.sapling_bundle {
        zcashd::SaplingBundle::V4(bundle_v4) => {
            for spend in bundle_v4.spends() {
                // The nullifier uniquely identifies the spend
                // Use AsRef to get a reference to the underlying bytes
                let nullifier_hex = hex::encode(spend.nullifier().as_ref() as &[u8]);
                meta_addresses.insert(format!("sapling_spend:{}", nullifier_hex));

                // If we have note data for this nullifier, we might find the address
                if let Some(sapling_note_data) = &tx.sapling_note_data {
                    for note_data in sapling_note_data.values() {
                        if let Some(nullifier) = note_data.nullifer() {
                            if *nullifier == spend.nullifier() {
                                // Find the address for this viewing key
                                for (addr, ivk) in wallet.sapling_z_addresses() {
                                    if note_data.incoming_viewing_key() == ivk {
                                        let addr_str = addr.to_string(wallet.network());
                                        addresses.insert(AddressId::Sapling(addr_str));
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }

            for output in bundle_v4.outputs() {
                // The commitment uniquely identifies the output
                // Use AsRef to get a reference to the underlying bytes
                let cm_hex = hex::encode(output.cmu().as_ref() as &[u8]);
                meta_addresses.insert(format!("sapling_output:{}", cm_hex));

                // If we have note data for this output, we might find the address
                if let Some(sapling_note_data) = &tx.sapling_note_data {
                    for note_data in sapling_note_data.values() {
                        // We'd need to link the outpoint to this specific output
                        // Since we don't have enough information, we'll use the IVK
                        for (addr, ivk) in wallet.sapling_z_addresses() {
                            if note_data.incoming_viewing_key() == ivk {
                                let addr_str = addr.to_string(wallet.network());
                                addresses.insert(AddressId::Sapling(addr_str));
                                break;
                            }
                        }
                    }
                }
            }
        }
        zcashd::SaplingBundle::V5(bundle_v5) => {
            // Similar processing for V5 bundles
            // V5 has the same structure for spends and outputs
            for spend in bundle_v5.shielded_spends() {
                // Use AsRef to get a reference to the underlying bytes
                let nullifier_hex = hex::encode(spend.nullifier().as_ref() as &[u8]);
                meta_addresses.insert(format!("sapling_spend_v5:{}", nullifier_hex));
            }

            for output in bundle_v5.shielded_outputs() {
                // Use AsRef to get a reference to the underlying bytes
                let cm_hex = hex::encode(output.cmu().as_ref() as &[u8]);
                meta_addresses.insert(format!("sapling_output_v5:{}", cm_hex));
            }
        }
    }

    // Process sapling note data directly
    if let Some(sapling_note_data) = &tx.sapling_note_data {
        for note_data in sapling_note_data.values() {
            // If we have the incoming viewing key, we can find the corresponding address
            for (addr, ivk) in wallet.sapling_z_addresses() {
                if note_data.incoming_viewing_key() == ivk {
                    let addr_str = addr.to_string(wallet.network());
                    addresses.insert(AddressId::Sapling(addr_str));
                    break;
                }
            }
        }
    }

    // Handle Orchard actions if present
    if let zcashd::OrchardBundle(Some(orchard_bundle)) = &tx.orchard_bundle {
        // Extract data from Orchard actions
        for (idx, action) in orchard_bundle.actions.iter().enumerate() {
            // Add standard identifiers like nullifier and commitment
            let nullifier_hex = hex::encode(action.nf_old);
            meta_addresses.insert(format!("orchard_nullifier:{}", nullifier_hex));

            // Extract potential address information if available
            if let Some(orchard_meta) = &tx.orchard_tx_meta {
                if let Some(_action_data) = orchard_meta.action_data.get(&(idx as u32)) {
                    // Try to recover the Orchard address components if we have enough data
                    // We don't have direct access to the receiver's complete address data here,
                    // but in a complete implementation, we'd follow the path:
                    // output_data -> full viewing key -> orchard receiver -> derive address

                    // For now, since we're missing the full path, we'll use what we have
                    // to create a unique identifier that links to metadata
                    let output_id = format!("orchard_output:{}:{}", tx_id, idx);
                    meta_addresses.insert(output_id);

                    // If the outgoing viewing key is related to ours, we can possibly
                    // derive additional information, but that's complex and
                    // would be out of scope for this implementation
                }
            }

            // Include any Orchard address we know of directly (e.g., from recipient mappings)
            // which would have been added already when processing recipient_mappings

            // Also add the action index as a unique identifier
            meta_addresses.insert(format!("orchard_action_idx:{}:{}", tx_id, idx));
        }
    }

    // If the transaction is marked as "from me", and we don't have other identifying information,
    // use all our addresses as potential sources
    if tx.from_me && addresses.is_empty() {
        for addr in wallet.sapling_z_addresses().keys() {
            let addr_str = addr.to_string(wallet.network());
            addresses.insert(AddressId::Sapling(addr_str));
        }

        // Also add transparent addresses if any are associated with this wallet
        for addr in wallet.address_names().keys() {
            addresses.insert(AddressId::Transparent(addr.0.clone()));
        }
    }

    // Add the transaction ID itself as a last resort identifier
    meta_addresses.insert(format!("tx:{}", tx_id));

    // If we didn't find any direct addresses, but have metadata addresses,
    // try to look up any addresses that could be related to these transaction components
    // This is just a stub for future implementation where we'd have a more
    // sophisticated lookup mechanism

    Ok(addresses)
}
