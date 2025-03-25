# CLAUDE.md - ZCash Wallet Export/Import Format Guidelines

## Project Overview

This project consists of four related crates in the `bc-rust` workspace:
- `zmigrate`: Command line tool to migrate wallets between different ZCash wallet formats
- `zewif`: Defines the ZeWIF interchange format for ZCash wallets using a common in-memory and serialized representation
- `zewif-zcashd`: Provides ZCashd-specific structures and migration code
- `zewif-zingo`: Provides Zingo-specific structures and migration code

🚨 NOTE: Only make changes to these four crates and no others in the workspace.

🚨 NOTE: The `target` directory is located outside the workspace. For dependency information, ask for assistance or refer to online crate documentation.

## Purpose and Design Philosophy

The ZeWIF format and zmigrate tool are designed for **data interchange**, not operational wallet functionality:

- **Purpose**: Enable migration between different ZCash wallet implementations
- **Focus**: Data preservation and format conversion, not wallet implementation
- **Future Plans**: Will eventually use Gordian Envelope for attachments (not implemented yet)

### Key Design Goals

1. **Wallet Migration**: Convert wallets between different implementations
2. **Data Preservation**: Ensure no critical wallet data is lost during transfers
3. **Interoperability**: Create a universal format that works with any ZCash wallet
4. **Key Recovery**: Facilitate recovery of keys and addresses from various wallet formats

### Design Approach

- Uses **optional fields** throughout to handle differences between wallet implementations
- Only populates fields when the corresponding data exists in the source wallet
- Preserves all data necessary for spending while maintaining interoperability
- Prioritizes preserving critical data (keys, addresses) while handling wallet-specific data appropriately

## Key Types and Resources

### Core Types in `zewif` Crate

| Type | Description |
|------|-------------|
| `Blob<N>` | Fixed-size byte array (wrapper around `[u8; N]`) |
| `Data` | Variable-size byte array (wrapper around `Vec<u8>`) |
| `u256` | 256-bit unsigned integer, little-endian byte order |
| `u252` | 252-bit unsigned integer for Orchard note commitments |
| `u160` | 160-bit unsigned integer for transparent addresses |
| `TxId` | Transaction ID (32 bytes representing a transaction hash) |
| `ZewifTop` | Top-level container for wallet data |
| `ZewifWallet` | Complete wallet representation |
| `Account` | Account within a wallet |
| `Address` | ZCash address (transparent, shielded, or unified) |
| `Transaction` | ZCash transaction |
| `Position` | Position in a note commitment tree (for spending notes) |

### Resources

- Original `zcashd` code: `reference-code/zcash-master` directory
- Transaction assignment reference: `zewif-zcashd/docs/TransactionAssignment.md`

## Development Guidelines

### Code Style and Architecture

- **NO Gordian Envelope attachments yet** - Will be added later for wallet-specific data
- **Use accessors, not pub fields** - All struct fields should be private with accessor methods
- **Production quality** - Write code as you would for a real-world implementation
- **No placeholders** - Implement full functionality or use `todo!()` for out-of-scope paths
- **Proper error handling** - Use `Result<T>` with `anyhow::Context` for all functions that can fail
- **Fail fast** - Return early if a function cannot proceed due to an error
- **Clean code** - Fix all compiler errors and Clippy lints in your modified crates

### Testing

- Add tests to `zmigrate/tests` directory
- Test fixtures are stored in `zmigrate/tests/fixtures/`
- Don't mark tasks as complete until all tests pass

## Current Implementation Status

### Successfully Implemented Features

✅ **Core Wallet Structure**
- Basic wallet components fully migrated
- Account structure properly created and maintained
- 100% of transactions preserved across migration
- All addresses preserved and correctly mapped

✅ **Transaction Processing**
- 100% transaction assignment to correct accounts
- Transaction metadata (status, timestamps, block hashes) preserved
- Witness data properly extracted and preserved
- Note commitment trees migrated with position information

✅ **Key Management**
- Spending keys successfully migrated
- Incoming viewing keys preserved
- Key-to-address relationships maintained

### Areas for Improvement

⚠️ **Memo Data Handling**
- Memo data preserved in encrypted form
- Not directly accessible without decryption keys (by design)
- Requires receiving wallet to handle decryption

⚠️ **Blockchain Data Access**
- Block hashes preserved but heights need blockchain access
- Complete validation requires blockchain interaction

⚠️ **Viewing Key Support**
- Currently limited to incoming viewing keys
- Full viewing keys need proper implementation
- Key hierarchies need better support

## Task Roadmap

🚨 NOTE: All tasks must focus strictly on data preservation and migration rather than operational wallet functionality.

### ✅ COMPLETED TASKS

**Transaction Conversion and Metadata**
- ✅ Witness data extraction and preservation in ZeWIF format
- ✅ Memo field preservation in encrypted ciphertexts
- ✅ Transaction metadata extraction (status, timestamps, block hashes)
- ✅ Transaction assignment (100% success rate)
- ✅ Note commitment tree migration

### 🔴 CURRENT PRIORITY: Viewing Key Preservation

**Why Critical**: Viewing keys are essential wallet data that must be preserved during migration to maintain data completeness.

**Required Tasks:**
1. ⬜ Extract full viewing keys from source wallets
2. ⬜ Implement proper data structures to store this information in ZeWIF format
3. ⬜ Preserve associations between viewing keys and their corresponding addresses
4. ⬜ Maintain any key hierarchy metadata from the original wallet

**Implementation Approach:**
- Focus on correctly extracting viewing key data from source wallets
- Add appropriate storage in the interchange format
- Preserve key-to-address mappings for data integrity

### 🟠 SECONDARY PRIORITIES

**Unified Address Data Preservation**
- ⬜ Extract and preserve unified addresses with multiple receiver types
- ⬜ Preserve diversifier indices from source wallets
- ⬜ Support all receiver types including Orchard receivers
- ⬜ Add tests to verify complete address data preservation

**Address and Key Relationship Preservation**
- ⬜ Preserve transparent address derivation paths
- ⬜ Extract and store key origin information
- ⬜ Maintain HD path data for complete derivation information

## Build/Test Commands

- Build project: `cargo build`
- Run project: `cargo run -- [zcash|zingo] path/to/wallet.dat > wallet-dump.txt`
- Check compilation: `cargo check`
- Run with traces: `cargo run --features with-context -- [zcash|zingo] path/to/wallet.dat`
- Run clippy lints: `cargo clippy -- -D warnings`
- Format code: `cargo fmt`
- Run specific tests: `cargo test --test test_transaction_assignment`
