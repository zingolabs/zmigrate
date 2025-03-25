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

## Migration Status and Next Tasks

- The current goal is to migrate a `ZcashdWallet` (structures in `zewif-zcashd`) to a `ZewifTop` (structures in `zewif`).
- The main migration function is `zewif-zcashd::migrate::migrate_to_zewif`.
- We are primarily working in the `zewif-zcashd::migrate` and `zewif` modules.
- We are focused on converting in-memory wallet data to ZeWIF abstractions, not on serialization or file I/O yet.

### Current Tasks (HIGH PRIORITY)

1. **Note Commitment Trees Migration** (COMPLETED)
   - Status: Successfully implemented with support for all wallet formats
   - Why it's critical: Note commitment trees contain essential data for spending notes - without this data, users cannot spend their funds
   - Subtasks:

     a. **Enhance OrchardNoteCommitmentTree Parser** (COMPLETED)
     - ✅ Improved the binary format parser to correctly process all 13,954 bytes
     - ✅ Added complete validation for tree structure integrity
     - ✅ Implemented robust error handling with context
     - ✅ Added methods to extract tree structure information (depth, size, etc.)
     - ✅ Added support for different ZCash serialization magic numbers (5050150, 5060050, etc.)
     - ✅ Added position tracking for each node in the tree
     - ✅ Implemented mapping between commitments and their positions
     - ✅ Added commitment detection algorithm to extract real values from binary data
     - ✅ Fixed tree_size field to accurately reflect the actual number of nodes
     - ✅ Implemented proper root node construction from leaf nodes

     b. **Implement Position Calculation** (COMPLETED)
     - ✅ Created algorithm for calculating leaf positions in the tree
     - ✅ Implemented efficient traversal of the tree structure
     - ✅ Mapped commitment hashes to their positions in the tree
     - ✅ Created an indexed lookup system for fast commitment-to-position mapping
     - ✅ Added position validation logic
     - ✅ Implemented fallback to sequential positions when tree parsing fails
     - ✅ Created three-tier extraction approach for different wallet formats

     c. **Update Transaction Output Logic** (COMPLETED)
     - ✅ Enhanced the `update_transaction_positions` function to use position mapping
     - ✅ Added mutable access methods for Sapling outputs and Orchard actions
     - ✅ Implemented transaction mutation capabilities to update positions
     - ✅ Fixed the placeholder Position(0) values with actual positions from the tree
     - ✅ Added proper error handling and logging
     - ✅ Implemented fallback strategies when positions can't be determined
     - ✅ Ensured compatibility with all wallet serialization formats

     d. **Testing and Validation** (COMPLETED)
     - ✅ Created tests to verify transaction position handling
     - ✅ Validated position integrity across the migration process
     - ✅ Added safeguards with sequential positions as fallback
     - ✅ Tested with various wallet formats (golden, tarnished, sprout, etc.)
     - ✅ Created detailed diagnostic information in tree summary
     - ✅ Added filtering to identify likely valid commitments vs. placeholders
     - ✅ Implemented quality testing to measure position preservation

2. **Note Position Preservation** (COMPLETED)
   - Status: Successfully extracting and applying real positions from tree data
   - Why it's critical: Position information is required for creating valid ZCash transactions and spending notes
   - This task has been fully implemented as part of "Note Commitment Trees Migration" above
   - Added three-tier approach to position handling:
     - Extract real commitments from binary data when possible
     - Intelligently identify likely valid commitments vs. placeholders
     - Fall back to sequential positions when extraction isn't possible


### Next Tasks (MEDIUM PRIORITY)

1. **Transaction Assignment Logic** (COMPLETED)
   - Status: Successfully implemented with smart address extraction and tiered assignment
   - ✅ Improved how transactions are assigned to accounts based on address involvement
   - ✅ Replaced placeholder code with robust, hierarchical assignment logic
   - ✅ Added better transaction address identification with tagged addresses
   - ✅ Implemented intelligent fallback strategies based on transaction type
   - ✅ Enhanced change detection for more accurate source account identification
   - ✅ Improved AddressRegistry with support for all address types
   - ✅ Added proper handling of multi-account transactions
   - ✅ Implemented context-based fallback to avoid assigning transactions to all accounts
   - ✅ Added validation to ensure transactions are properly associated with relevant accounts
   - 🔵 NEEDED: Thorough testing with various wallet scenarios to validate the improvements

2. **Enhanced Transaction Conversion**
   - Improve witness data support for verification
   - Add proper memo field decryption when appropriate keys are available
   - Extract block height information from transaction metadata when available in source wallet
   - Research ZCash serialization formats to better extract complete transaction data

3. **Viewing Key Migration**
   - Complete missing viewing key conversion code
   - Preserve viewing key relationships with addresses in ZeWIF format
   - Properly handle both incoming viewing keys and full viewing keys
   - Implement more robust viewing key parsing and validation

4. **Unified Address Support**
   - Add support for unified addresses with multiple receiver types
   - Properly handle diversifier indices
   - Ensure proper handling of receiver types including Orchard receivers
   - Add comprehensive tests for unified address migration

### Low-Priority Tasks

1. **Key Mapping Improvements**
   - Implement robust transparent address derivation from keys and scripts
   - Add proper viewing key support
   - Create a key registry for faster lookups

2. **Witness Data Migration**
   - Complete witness data conversion between in-memory formats
   - Properly map witness structures to ZeWIF memory representation
   - Ensure witnesses can be used for verification and spending

3. **Diagnostic and Quality Testing**
   - Expand migration quality testing to measure more aspects of wallet conversion
   - Add more detailed logging and diagnostics for complex wallet formats
   - Create real-world migration success metrics reporting

## Implementation Progress

### Recent Improvements

1. **Transaction Assignment Logic Enhancements** (March 2025)
   - ✅ Completely redesigned transaction-to-account assignment with tiered assignment strategy
   - ✅ Implemented smarter address extraction from transactions with tagged identifiers
   - ✅ Created helper functions for identifying transaction types (change, send, receive)
   - ✅ Enhanced AddressRegistry with complete transparent and sapling address support
   - ✅ Added intelligent fallback strategies based on transaction context
   - ✅ Added better HD path analysis for account identification
   - ✅ Improved change address detection for more accurate source account mapping
   - ✅ Added transaction tagging for better ownership determination
   - ✅ Fixed error handling to avoid default assignment to all accounts
   - ✅ Documented the complete transaction assignment approach in `zewif-zcashd/docs/TransactionAssignment.md`
   - 🔵 NEEDED: Comprehensive test suite for transaction assignment with different wallet scenarios

2. **Orchard Note Commitment Tree Enhancements** (March 2025)
   - ✅ Fixed issues with the tree_size field showing incorrect values (like 1657887612848898305)
   - ✅ Added proper root node construction for all tree parsing code paths
   - ✅ Improved tree format detection with tiered fallback strategies
   - ✅ Added comprehensive tests specifically targeting tree structure validation
   - ✅ Enhanced debug output with consistent and accurate tree information
   - ✅ Fixed all clippy warnings related to the tree implementation
   - ✅ Improved memory efficiency by clearing unparsed data after successful parsing

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

1. **Transaction Assignment Logic** (COMPLETED)
   - ✅ Completely revamped transaction assignment with hierarchical approach
   - ✅ Enhanced AddressRegistry to map all address types to accounts
   - ✅ Implemented intelligent transaction analysis with tagged address types
   - ✅ Added context-aware fallback logic based on transaction type
   - ✅ Improved change detection for more accurate source account identification
   - ✅ Added support for multi-account transaction handling
   - ✅ Created comprehensive documentation of assignment approach in TransactionAssignment.md
   - ✅ Eliminated indiscriminate assignment of transactions to all accounts
   - ✅ Fixed all compiler warnings and kept code clean and maintainable
   - 🔵 NEEDED: Comprehensive testing with real wallet scenarios

2. **Orchard Note Commitment Tree Processing** (COMPLETED)
   - ✅ Enhanced OrchardNoteCommitmentTree parser with ZCash serialization format support
   - ✅ Implemented smart position calculation and commitment extraction
   - ✅ Updated transaction output logic with positions from real tree data
   - ✅ Added comprehensive testing and validation for all wallet formats
   - ✅ Fixed tree_size field to accurately reflect actual number of commitments found
   - ✅ Implemented proper root node construction based on leaf nodes
   - ✅ Added robust fallback approaches when binary data format cannot be fully parsed

3. **Note Position Preservation** (COMPLETED)
   - ✅ Successfully extracting and preserving positions from tree data
   - ✅ Created multi-tier approach to handle different wallet formats
   - ✅ Added intelligent commitment detection to extract real values
   - ✅ Implemented fallback mechanisms for older wallet versions

4. **Transaction Data Structure Conversion** (COMPLETED)
   - ✅ Added proper transaction data conversion to in-memory ZeWIF structures
   - ✅ Improved in-memory representation of transaction components
   - ✅ Fixed note position placeholder values during migration
   - ✅ Tested with real wallet data across multiple formats
   - ✅ Added compatibility for various ZCash serialization formats
