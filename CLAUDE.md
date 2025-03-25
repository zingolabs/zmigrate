# CLAUDE.md - ZCash Wallet Export/Import Format Guidelines

- You are working on several related crates in the `bc-rust` workspace:
  - `zmigrate` is a command line tool to migrate wallets between different ZCash wallet formats.
  - `zewif` defines the ZeWIF interchange format for ZCash wallets, a common in-memory and serialized representation for wallet data based on Gordian Envelope.
  - `zewif-zcashd` provides ZCashd-specific structures and migration code for the `zmigrate` tool.
  - `zewif-zingo` provides Zingo-specific structures and migration code for the `zmigrate` tool.
  - DO NOT MAKE CHANGES OUTSIDE THESE FOUR CRATES.

- You will *not* find a `target` directory in `bc-rust` because it lives outside the workspace. If you need to examine dependencies, let me know and I will provide you with the necessary information, or you can do a web search for the crate documentation.

## Introduction

- The `zmigrate` tool and `zewif` framework enable migration between different ZCash wallet implementations.
- The ZeWIF interchange format is *not* intended to provide a full wallet implementation. It is a framework for the migration of wallets between different wallet implementations.
- The ZeWIF file format will be based on Gordian Envelope, with wallet-specific data stored in attachments. NOTE: We are *not* writing code using attachments yet.
- The `zmigrate` command line tool converts wallets to and from ZeWIF files, enabling conversion between wallet formats.

### Purpose of ZeWIF and ZMigrate

The `zmigrate` tool and `zewif` framework serve several key purposes:

1. **Wallet Migration** - Enables users to convert their existing wallets from specific implementations to a universal format
2. **Wallet Interoperability** - Allows wallet data to be moved between different ZCash wallet implementations
3. **Data Preservation** - Ensures no critical wallet data is lost during transfers between implementations
4. **Key Recovery** - Facilitates recovery of keys and addresses from various wallet formats

### ZeWIF Format Design Notes

- ZeWIF uses **optional fields** throughout the format to accommodate differences between wallet implementations
- Fields are populated only when the corresponding data exists in the source wallet
- Fields like `raw` transaction data, `mined_height`, etc. may be `None` for wallet formats that don't include this data
- Critical data like keys and addresses will always be migrated, while wallet-implementation-specific data might be omitted
- The goal is to preserve all data necessary for spending and verification while maintaining wallet interoperability

## Important Basic Types Provided by the `zewif` Crate

- `Blob<N>`: A fixed-size byte array (wrapper around `[u8; N]`)
- `Blob32`: Type alias for `Blob<32>`
- `Data`: A variable-size byte array (wrapper around `Vec<u8>`)
- `u256`: A 256-bit unsigned integer (wrapper around `[u8; 32]`), assumes little-endian byte order, implements `Copy`
- `u252`: A 252-bit unsigned integer (wrapper around `[u8; 32]`), used for Orchard note commitments, implements `Copy`
- `u160`: A 160-bit unsigned integer (wrapper around `[u8; 20]`), used for transparent addresses, implements `Copy`
- `TxId`: A transaction ID, which is 32 bytes representing a transaction hash
- `ZewifTop`: The top-level container for wallet data
- `ZewifWallet`: Represents a complete wallet
- `Account`: Represents an account within a wallet
- `Address`: Represents a ZCash address (transparent, shielded, or unified)
- `Transaction`: Represents a ZCash transaction
- `Position`: Represents a position in a note commitment tree, essential for spending notes

## Resources

- The entire code for the original `zcashd` is in `reference-code/zcash-master`.
- A detailed reference guide for how `zcashd` performs transaction assignment is in `zewif-zcashd/docs/TransactionAssignment.md`.

## Coding Notes

- Do *not* use the Gordian Envelope attachments feature yet. It will be used later to preserve wallet-specific data.
- When adding overall tests, add them to `zmigrate/tests` and use the `zmigrate/tests/fixtures/` directory for test data from actual wallets.
- Don't define structures with `pub` fields. Use accessors instead.
- If you think, "In a *real* implementation we'd do it this way," then do it that way. We're not doing coding exercises, this is production code.
- Do not use placeholder implementations when writing new code; implement the full functionality. If a particular code path is out of scope, mark it with a `todo!()` macro.
- Use `Result<T>` and proper error handling with context (`anyhow::Context`) for all functions that can fail.
- Always prefer "fail fast" error handling. If a function cannot proceed due to an error, return early.
- Make sure all compiler errors and Clippy lints are fixed in crates you modify.
- Don't mark items complete below until they are fully implemented, including fixing compile errors, clippy lints, and failing tests.

## Migration Status and Analysis

Based on analysis of the current ZCashd wallet migration to ZeWIF format, the following is the status summary:

### What's Working Well
- **Basic Wallet Structure**: All wallet structure components are successfully migrated
- **Transaction Preservation**: 100% of transactions are preserved (139/139 in test wallet)
- **Address Preservation**: Addresses are preserved and correctly mapped (6 addresses in ZeWIF vs 1 address in ZCashd - likely due to more complete address generation)
- **Account Structure**: Accounts are properly created and structured in the ZeWIF format
- **Transaction Assignment**: Transactions are correctly assigned to appropriate accounts (100% assignment rate)
- **Note Commitment Trees**: Successfully migrated with proper position information
- **Key Preservation**: Spending keys and incoming viewing keys are successfully migrated

### What Needs Improvement
- **Memo Fields**: All memo fields currently show as `None` in the output
- **Block Height Information**: More detailed transaction metadata like block heights needs proper extraction
- **Witness Data**: All witness fields currently show as `None` in the output
- **Full Viewing Key Support**: Need to enhance viewing key migration beyond just incoming viewing keys

## Current Tasks

### HIGH PRIORITY

1. **Enhanced Transaction Conversion**
   - Status: Needs Implementation
   - Why it's critical: Complete transaction metadata is essential for proper wallet functionality
   - Subtasks:
     a. **Implement Witness Data Support**
       - ⬜ Add proper witness data extraction from source wallet
       - ⬜ Implement witness data conversion to ZeWIF format
       - ⬜ Create validation to ensure witness data integrity
     
     b. **Add Memo Field Support**
       - ⬜ Implement memo field extraction from transaction data
       - ⬜ Add memo decryption when appropriate keys are available
       - ⬜ Create proper memo field representation in ZeWIF format
     
     c. **Extract Transaction Metadata**
       - ⬜ Extract and preserve block height information
       - ⬜ Preserve transaction timestamp data when available
       - ⬜ Add proper transaction status information

2. **Viewing Key Migration**
   - Status: Partially Implemented
   - Why it's critical: Viewing keys are essential for transaction history without spending capability
   - Subtasks:
     a. **Complete Viewing Key Support**
       - ⬜ Properly handle both incoming viewing keys and full viewing keys
       - ⬜ Preserve viewing key relationships with addresses
       - ⬜ Implement comprehensive viewing key validation
     
     b. **Add Viewing Key Derivation Logic**
       - ⬜ Implement logic for deriving viewing keys from spending keys
       - ⬜ Ensure proper key hierarchies are maintained

### MEDIUM PRIORITY

1. **Unified Address Support**
   - Status: Basic Support Implemented
   - Subtasks:
     - ⬜ Add support for unified addresses with multiple receiver types
     - ⬜ Properly handle diversifier indices
     - ⬜ Ensure proper handling of receiver types including Orchard receivers
     - ⬜ Add comprehensive tests for unified address migration

2. **Key Mapping Improvements**
   - Status: Basic Implementation Complete
   - Subtasks:
     - ⬜ Implement robust transparent address derivation from keys and scripts
     - ⬜ Create a key registry for faster lookups
     - ⬜ Enhance HD path analysis for more accurate account determination

### COMPLETED TASKS

1. **Transaction Assignment Logic**
   - Status: Successfully implemented and comprehensively tested
   - ✅ Implemented hierarchical transaction assignment strategy
   - ✅ Created complete address-to-account mapping system
   - ✅ Added intelligent fallback strategies based on transaction type
   - ✅ Implemented comprehensive test suite with 100% transaction assignment success rate

2. **Note Commitment Trees Migration**
   - Status: Successfully implemented with support for all wallet formats
   - ✅ Enhanced binary format parsing for tree structures
   - ✅ Implemented position calculation and preservation
   - ✅ Created three-tier approach to handle different wallet formats
   - ✅ Added fallback mechanisms for older wallet versions

## Build/Test Commands

- Build project: `cargo build`
- Run project: `cargo run -- [zcash|zingo] path/to/wallet.dat > wallet-dump.txt`
- Check compilation: `cargo check`
- Run with traces: `cargo run --features with-context -- [zcash|zingo] path/to/wallet.dat`
- Run clippy lints: `cargo clippy -- -D warnings`
- Format code: `cargo fmt`
- Run specific tests: `cargo test --test test_transaction_assignment`