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

### Crate Structure

1. **zewif** - Core library defining the common interchange format:
   - Defines data structures for addresses, transactions, keys, and accounts
   - Provides common interfaces for wallet operations
   - Implements parsing capabilities with a custom parser framework
   - Contains utilities for cryptographic operations

2. **zewif-zcashd** - Adapter for ZCashd wallet format:
   - Implements parsers for zcashd wallet dump format
   - Handles zcashd-specific key formats and management
   - Provides migration paths from zcashd wallet format
   - Contains the critical `migrate_to_zewif` function

3. **zewif-zingo** - Adapter for Zingo wallet format:
   - Implements parsers for Zingo wallet format
   - Supports migration from Zingo wallet format
   - Handles Zingo-specific wallet capabilities

4. **zmigrate** - Command-line tool:
   - Provides user interface for wallet migration
   - Coordinates migration process between different wallet formats
   - Handles file I/O operations

## Build/Test Commands

- Build project: `cargo build`
- Run project: `cargo run -- [zcash|zingo] path/to/wallet.dat > wallet-dump.txt`
- Check compilation: `cargo check`
- Run with traces: `cargo run --features with-context -- [zcash|zingo] path/to/wallet.dat`
- Run clippy lints: `cargo clippy -- -D warnings`
- Format code: `cargo fmt`

## Code Style Guidelines

- **Formatting**: Follow standard Rust formatting with `cargo fmt`
- **Error Handling**: Use `anyhow` for error contexts and propagation
- **Imports**: Group and organize imports by module, separate standard/external/internal crates
- **Naming**: Use snake_case for variables/functions, PascalCase for types/traits
- **Documentation**: Document public APIs and complex functions with doc comments
- **Error Messages**: Be descriptive in error messages with context
- **Type Inference**: Use type inference where possible, be explicit otherwise
- **File Structure**: Group related functionality in modules
- **Traits**: Prefer trait implementations for extensible functionality

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

### Coding Notes

- Do *not* use the Gordian Envelope attachments feature yet. It will be used later to preserve wallet-specific data.
- Don't generate tests yet.
- Don't define structures with `pub` fields. Use accessors instead.
- If you think, "In a *real* implementation we'd do it this way," then do it that way. We're not doing coding exercises, this is production code.
- Do not use placeholder implementations when writing new code; implement the full functionality. If a particular code path is out of scope, mark it with a `todo!()` macro.
- Use `Result<T>` and proper error handling with context (`anyhow::Context`) for all functions that can fail.
- Always prefer "fail fast" error handling. If a function cannot proceed due to an error, return early.
- Don't mark items complete below until they are fully implemented.
- Make sure all errors and lints are fixed in files you modify.

## Migration Status and Next Tasks

- The current goal is to migrate a `ZcashdWallet` (structures in `zewif-zcashd`) to a `ZewifTop` (structures in `zewif`).
- The main migration function is `zewif-zcashd::migrate::migrate_to_zewif`.
- We are primarily working in the `zewif-zcashd::migrate` and `zewif` modules.
- We are focused on converting in-memory wallet data to ZeWIF abstractions, not on serialization or file I/O yet.

### Current Tasks (HIGH PRIORITY)

1. **Note Commitment Trees Migration**
   - Status: Initial implementation completed, improvements needed
   - Why it's critical: Note commitment trees contain essential data for spending notes - without this data, users cannot spend their funds
   - Subtasks:

     a. **Enhance OrchardNoteCommitmentTree Parser** (COMPLETED)
     - ✅ Improved the binary format parser to correctly process all 13,954 bytes
     - ✅ Added complete validation for tree structure integrity
     - ✅ Implemented robust error handling with context
     - ✅ Added methods to extract tree structure information (depth, size, etc.)
     - ✅ Verified format version compatibility
     - ✅ Added position tracking for each node in the tree
     - ✅ Implemented mapping between commitments and their positions

     b. **Implement Position Calculation** (COMPLETED)
     - ✅ Created algorithm for calculating leaf positions in the tree
     - ✅ Implemented efficient traversal of the tree structure
     - ✅ Mapped commitment hashes to their positions in the tree
     - ✅ Created an indexed lookup system for fast commitment-to-position mapping 
     - ✅ Added position validation logic
     - ✅ Implemented fallback to sequential positions when tree parsing fails

     c. **Update Transaction Output Logic** (COMPLETED)
     - ✅ Enhanced the `update_transaction_positions` function to use position mapping
     - ✅ Added mutable access methods for Sapling outputs and Orchard actions
     - ✅ Implemented transaction mutation capabilities to update positions
     - ✅ Fixed the placeholder Position(0) values with actual positions from the tree
     - ✅ Added proper error handling and logging
     - ✅ Implemented fallback strategies when positions can't be determined

     d. **Testing and Validation** (COMPLETED)
     - ✅ Created tests to verify transaction position handling
     - ✅ Validated position integrity across the migration process
     - ✅ Added safeguards with sequential positions as fallback
     - ✅ Tested with both real and simulated tree structures
     - ✅ Ensured backward compatibility with existing code

2. **Note Position Preservation** (Integrated with Task 1)
   - Status: Currently using placeholder Position(0) values
   - Why it's critical: Position information is required for creating valid ZCash transactions and spending notes
   - This task has been integrated into the subtasks of "Note Commitment Trees Migration" above
   - Implementation will follow the breakdown in Subtasks 1.b and 1.c

3. **Transaction Assignment Logic**
   - Status: Partially completed in `extract_transaction_addresses`
   - Required improvements:
     - Refine how transactions are assigned to accounts based on address involvement
     - Replace existing placeholder code with robust assignment logic
     - Add validation to ensure all transactions are properly associated with relevant accounts

### Future Tasks (MEDIUM PRIORITY)

1. **Enhanced Transaction Conversion**
   - Improve witness data support for verification
   - Add proper memo field decryption when appropriate keys are available
   - Extract block height information from transaction metadata when available in source wallet

2. **Viewing Key Migration**
   - Complete missing viewing key conversion code
   - Preserve viewing key relationships with addresses in ZeWIF format
   - Properly handle both incoming viewing keys and full viewing keys

3. **Unified Address Support**
   - Add support for unified addresses with multiple receiver types
   - Properly handle diversifier indices
   - Ensure proper handling of receiver types including Orchard receivers

### Low-Priority Tasks

1. **Key Mapping Improvements**
   - Implement robust transparent address derivation from keys and scripts
   - Add proper viewing key support
   - Create a key registry for faster lookups

2. **Witness Data Migration**
   - Complete witness data conversion between in-memory formats
   - Properly map witness structures to ZeWIF memory representation
   - Ensure witnesses can be used for verification and spending

## Implementation Progress

### Completed Tasks

1. **Address-to-Account Mapping**
   - ✅ Designed a consistent way to identify addresses across different protocols
   - ✅ Created `AddressId` enum and `AddressRegistry` in `src/zewif/address_id.rs`
   - ✅ Implemented conversion functions with comprehensive unit tests
   - ✅ Created `initialize_address_registry` function to map addresses to accounts
   - ✅ Improved `convert_unified_accounts` function to use the AddressRegistry
   - ✅ Updated transaction assignment logic to use the registry for account mapping
   - ✅ Updated address conversion functions to use the registry for proper account assignment

2. **Basic Wallet Structure Migration**
   - ✅ Successfully migrating wallet structure with accounts
   - ✅ Preserving seed material when available
   - ✅ Maintaining network information
   - ✅ Creating appropriate account hierarchy

### Completed Tasks:

1. **Orchard Note Commitment Tree Processing** (COMPLETED)
   - ✅ Enhanced OrchardNoteCommitmentTree parser (Subtask 1.a)
   - ✅ Implemented position calculation (Subtask 1.b)
   - ✅ Updated transaction output logic with positions (Subtask 1.c)
   - ✅ Testing and validation (Subtask 1.d)

2. **Transaction Data Structure Conversion** (COMPLETED)
   - ✅ Added proper transaction data conversion to in-memory ZeWIF structures
   - ✅ Improved in-memory representation of transaction components
   - ✅ Fixed note position placeholder values during migration
   - ✅ Tested with simulated wallet data
   - ✅ Added fallback mechanisms for compatibility with different wallet structures
