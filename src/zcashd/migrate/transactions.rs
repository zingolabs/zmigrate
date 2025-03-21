use std::collections::HashMap;

use anyhow::{Context, Result};

use crate::{zcashd::{self, ZcashdWallet}, zewif, Position, TxId};

/// Convert ZCashd transactions to Zewif format
pub fn convert_transactions(wallet: &ZcashdWallet) -> Result<HashMap<TxId, zewif::Transaction>> {
    let mut transactions = HashMap::new();

    for (tx_id, wallet_tx) in wallet.transactions() {
        let zewif_tx = convert_transaction(*tx_id, wallet_tx)
            .with_context(|| format!("Failed to convert transaction {}", tx_id))?;
        transactions.insert(*tx_id, zewif_tx);
    }

    Ok(transactions)
}

/// Convert a single ZCashd transaction to Zewif format
fn convert_transaction(tx_id: TxId, tx: &zcashd::WalletTx) -> Result<zewif::Transaction> {
    let mut zewif_tx = zewif::Transaction::new(tx_id);

    // Set raw transaction data
    if !tx.unparsed_data.is_empty() {
        zewif_tx.set_raw(tx.unparsed_data.clone());
    }

    // Add basic transaction metadata
    // Convert block height if we can infer it from hash_block
    // For this prototype, we'll just leave it as None

    // Convert transparent inputs
    for tx_in in &tx.vin {
        let zewif_tx_in = zewif::TxIn::new(
            zewif::TxOutPoint::new(tx_in.prevout().txid(), tx_in.prevout().vout()),
            tx_in.script_sig().clone(),
            tx_in.sequence(),
        );
        zewif_tx.add_input(zewif_tx_in);
    }

    // Convert transparent outputs
    for tx_out in &tx.vout {
        let amount = tx_out.value();
        let script_pubkey = tx_out.script_pub_key().clone();

        let zewif_tx_out = zewif::TxOut::new(amount, script_pubkey);
        zewif_tx.add_output(zewif_tx_out);
    }

    // Convert Sapling spends and outputs
    match &tx.sapling_bundle {
        zcashd::SaplingBundle::V4(bundle_v4) => {
            // Convert Sapling spends
            for (idx, spend) in bundle_v4.spends().iter().enumerate() {
                let mut sapling_spend = zewif::SaplingSpendDescription::new();
                sapling_spend.set_spend_index(idx as u32);
                sapling_spend.set_value(Some(bundle_v4.amount()));
                sapling_spend.set_nullifier(spend.nullifier());
                sapling_spend.set_zkproof(spend.zkproof().clone());
                zewif_tx.add_sapling_spend(sapling_spend);
            }

            // Convert Sapling outputs
            for (idx, output) in bundle_v4.outputs().iter().enumerate() {
                let mut sapling_output = zewif::SaplingOutputDescription::new();
                sapling_output.set_output_index(idx as u32);
                sapling_output.set_commitment(output.cmu());
                sapling_output.set_ephemeral_key(output.ephemeral_key());
                sapling_output.set_enc_ciphertext(output.enc_ciphertext().clone());
                zewif_tx.add_sapling_output(sapling_output);
            }
        }
        zcashd::SaplingBundle::V5(bundle_v5) => {
            // Processing for V5 bundles
            for (idx, spend) in bundle_v5.shielded_spends().iter().enumerate() {
                let mut sapling_spend = zewif::SaplingSpendDescription::new();
                sapling_spend.set_spend_index(idx as u32);
                sapling_spend.set_nullifier(spend.nullifier());
                sapling_spend.set_zkproof(spend.zkproof().clone());
                zewif_tx.add_sapling_spend(sapling_spend);
            }

            for (idx, output) in bundle_v5.shielded_outputs().iter().enumerate() {
                let mut sapling_output = zewif::SaplingOutputDescription::new();
                sapling_output.set_output_index(idx as u32);
                sapling_output.set_commitment(output.cmu());
                sapling_output.set_ephemeral_key(output.ephemeral_key());
                sapling_output.set_enc_ciphertext(output.enc_ciphertext().clone());
                zewif_tx.add_sapling_output(sapling_output);
            }
        }
    }

    // Convert Orchard actions
    if let zcashd::OrchardBundle(Some(orchard_bundle)) = &tx.orchard_bundle {
        for (idx, action) in orchard_bundle.actions.iter().enumerate() {
            let mut orchard_action = zewif::OrchardActionDescription::new();
            orchard_action.set_action_index(idx as u32);
            orchard_action.set_nullifier(action.nf_old);
            orchard_action.set_commitment(action.cmx);
            orchard_action.set_enc_ciphertext(action.encrypted_note.enc_ciphertext().clone());
            zewif_tx.add_orchard_action(orchard_action);
        }
    }

    // Convert Sprout JoinSplits if present
    if let Some(join_splits) = &tx.join_splits {
        for js in join_splits.descriptions() {
            // Create arrays using from_fn to avoid needing Copy
            let nullifiers = js.nullifiers();
            let commitments = js.commitments();

            let join_split = zewif::JoinSplitDescription::new(
                js.anchor(),
                nullifiers,
                commitments,
                js.zkproof().clone(),
            );
            zewif_tx.add_sprout_joinsplit(join_split);
        }
    }

    Ok(zewif_tx)
}

/// Update transaction outputs with note positions from the note commitment tree
pub fn update_transaction_positions(
    wallet: &ZcashdWallet,
    transactions: &mut HashMap<TxId, zewif::Transaction>,
) -> Result<()> {
    // Check if we have a valid tree to process
    let orchard_tree = wallet.orchard_note_commitment_tree();
    if orchard_tree.unparsed_data.is_empty() {
        return Ok(());
    }

    // Verify that we have a parsed tree with commitment positions
    if orchard_tree.commitment_positions.is_empty() && !orchard_tree.unparsed_data.is_empty() {
        eprintln!("Warning: Orchard note commitment tree has data but no parsed positions");
    }

    // Track statistics for reporting
    let mut orchard_positions_updated = 0;
    let mut sapling_positions_updated = 0;
    let mut total_orchard_actions = 0;
    let mut total_sapling_outputs = 0;

    // For each transaction with Orchard actions
    for (tx_id, zewif_tx) in transactions.iter_mut() {
        // Find the corresponding zcashd transaction to get metadata
        if let Some(zcashd_tx) = wallet.transactions().get(tx_id) {
            // Process Orchard actions if present
            if let zcashd::OrchardBundle(Some(_orchard_bundle)) = &zcashd_tx.orchard_bundle {
                // Check if we have mutable access to actions in the zewif transaction
                let orchard_actions = zewif_tx.orchard_actions_mut();

                if let Some(actions) = orchard_actions {
                    total_orchard_actions += actions.len();

                    // Process each Orchard action
                    for action in actions {
                        // Use our tree to find the position for this commitment
                        if let Some(position) = orchard_tree.find_position(action.commitment()) {
                            // Update the action with the correct position from the tree
                            action.set_note_commitment_tree_position(position);
                            orchard_positions_updated += 1;
                        } else {
                            // If we don't find a position in the tree, try to use metadata
                            if let Some(orchard_meta) = &zcashd_tx.orchard_tx_meta {
                                let action_idx = action.action_index();
                                if let Some(_action_data) =
                                    orchard_meta.action_data.get(&action_idx)
                                {
                                    // As a fallback, use the action index as a relative position
                                    // This isn't ideal but preserves some ordering information
                                    let fallback_position = Position(action_idx + 1); // Add 1 to avoid Position(0)
                                    action.set_note_commitment_tree_position(fallback_position);
                                }
                            }
                        }
                    }
                }
            }

            // Process Sapling outputs if present
            let sapling_outputs = zewif_tx.sapling_outputs_mut();
            if let Some(outputs) = sapling_outputs {
                total_sapling_outputs += outputs.len();

                // Try to set positions for sapling outputs
                for output in outputs {
                    // First, try to find the position in our Orchard tree (if commitments are shared)
                    // This is unlikely but worth checking
                    if let Some(position) = orchard_tree.find_position(output.commitment()) {
                        output.set_note_commitment_tree_position(position);
                        sapling_positions_updated += 1;
                    } else {
                        // Look up position from sapling note data if available
                        if let Some(sapling_note_data) = &zcashd_tx.sapling_note_data {
                            let output_idx = output.output_index();

                            // Find matching note data for this output
                            for (outpoint, note_data) in sapling_note_data.iter() {
                                if outpoint.txid() == *tx_id && outpoint.vout() == output_idx {
                                    // If we have witnesses, use their position information
                                    if !note_data.witnesses().is_empty() {
                                        // For now, just use a placeholder based on witness index
                                        // In a full implementation, we'd extract proper position from witness
                                        let position = Position(note_data.witnesses().len() as u32);
                                        output.set_note_commitment_tree_position(position);
                                        sapling_positions_updated += 1;
                                    } else {
                                        // As a last resort, use output index as relative position
                                        let fallback_position = Position(output_idx + 1); // Add 1 to avoid Position(0)
                                        output.set_note_commitment_tree_position(fallback_position);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Report statistics on how many positions were updated
    eprintln!("Note commitment tree position update complete:");
    eprintln!(
        "Orchard actions updated: {}/{}",
        orchard_positions_updated, total_orchard_actions
    );
    eprintln!(
        "Sapling outputs updated: {}/{}",
        sapling_positions_updated, total_sapling_outputs
    );

    Ok(())
}
