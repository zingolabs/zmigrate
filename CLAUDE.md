# CLAUDE.md - ZMigrate Guidelines

- You are working in the `zmigrate` crate.
- Above `zmigrate` is the containing `bc-rust` workspace.
- DO NOT MAKE CHANGES ABOVE THE `zmigrate` MODULE.
- You may examine the dependencies in the `bc-rust/target` directory there that we reference in this crate's `Cargo.toml` file.

## Introduction

- The `zmigrate` crate provides tooling to migrate wallets from various ZCash implementations (zcashd, zingo, etc.) to the common ZeWIF interchange format.
- NOTE: The zmigrate crate and the ZeWIF interchange format are *not* intended to provide a full wallet implementation. They are a tool to migrate wallets between different wallet implementations. The functionality provided by the `zmigrate` crate is strictly limited to wallet migration and data interchange.
- Eventually the wallet-specific parts will be moved to separate crates, which will provide front-end and back-end implementations for each wallet type.
- The ZeWIF file format itself, which will be directly supported by the `zmigrate` crate will be based on Gordian Envelope, with wallet-specific and esoteric data stored in attachments. NOTE: We are *not* writing code using attachments yet.
- The `zmigrate` command line tool will convert wallets to and from ZeWIF files, and will be able to convert between wallet formats.

### Purpose of ZMigrate

The `zmigrate` library serves several key purposes:

1. **Wallet Migration** - Enables users to convert their existing wallets from specific implementations to a universal format
2. **Wallet Interoperability** - Allows wallet data to be moved between different ZCash wallet implementations
3. **Data Preservation** - Ensures no critical wallet data is lost during transfers between implementations
4. **Key Recovery** - Facilitates recovery of keys and addresses from various wallet formats

### ZeWIF Interchange Format

ZeWIF (ZCash Wallet Interchange Format) is a specification designed to:

1. **Standardize Wallet Data** - Create a common representation of ZCash wallet data regardless of implementation
2. **Enable Cross-Wallet Compatibility** - Allow users to migrate between wallet implementations without losing data
3. **Future Proofing** - Provide a format that can accommodate future ZCash protocol developments
4. **Preserve All Critical Data** - Maintain spending keys, viewing keys, addresses, transactions, and metadata

The format is structured to capture the hierarchical nature of wallet data, from individual keys and addresses to accounts and complete wallets, while preserving the relationships between these elements.

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

## Important Basic Types

- `Blob<N>`: A fixed-size byte array (wrapper around `[u8; N]`)
- `Blob32`: Type alias for `Blob<32>`
- `Data`: A variable-size byte array (wrapper around `Vec<u8>`)
- `u256`: A 256-bit unsigned integer (wrapper around `[u8; 32]`), assumes little-endian byte order so reverses on display, usually used for cryptographic values. Implements `Copy`.
- `u252`: A 252-bit unsigned integer (wrapper around `[u8; 32]`), used for Orchard note commitments. Implements `Copy`.
- `u160`: A 160-bit unsigned integer (wrapper around `[u8; 20]`), used for transparent addresses. Implements `Copy`.

### Coding Notes

- Do *not* use the Gordian Envelope attachments feature yet. It will be used for wallet-specific data that is not part of the core ZeWIF interchange format, but which will be preserved by it.
- Don't generate tests yet.
- If you think, "In a *real* implementation we'd do it this way," then do it that way. We're not doing coding exercises, this is production code.
- Do not use and placeholder implementations when writing new code; implement the full functionality. If a particular code path is out of scope, mark it with a `todo!()` macro.
- Use `Result<T>` and proper error handling with context (using `anyhow::Context`) for all functions that can fail. We are using `anyhow` for error handling throughout.
- Always prefer "fail fast" error handling. If a function cannot proceed due to an error, return early.
- Don't mark items complete below until they are fully implemented.
- Make sure all errors and lints are fixed in files you modify.
- When recommending a next task:
  - Ensure that no other tasks are blocking the recommended task.
  - Determine whether the task requires any dependencies that aren't yet present.
  - Determine whether the task should be broken down into subtasks.

## Migration Plan: ZCashd to ZeWIF

### Migration Status and Next Tasks

The overall goal is to migrate a `ZcashdWallet` to a `ZewifTop` using the in-memory representations without focusing on serialization or file I/O at this time.

Current high-priority tasks:
1. **Unified Accounts Migration**: Convert the `wallet.unified_accounts` structure to ZeWIF format
2. **Sapling Note Commitment Trees Implementation**: Create a parser for Sapling note commitment trees

#### Current Tasks
- ✅ **Orchard Note Commitment Trees Migration** (COMPLETED)
  - ✅ Enhanced implementation for parsing and converting Orchard note commitment tree completed
  - ✅ Completed binary tree format parser with proper error handling
  - ✅ Added detailed mapping between commitments and their positions in the tree
  - ✅ Implemented mutable access methods to update transaction outputs with correct positions
  - ✅ Created proper witness data structures for each output:
    - ✅ Enhanced IncrementalMerkleTree with methods for authentication paths and tree inspection
    - ✅ Enhanced IncrementalWitness with proper initialization and witness creation
    - ✅ Added OrchardWitness type for specialized Orchard witness handling
    - ✅ Implemented witness conversion methods in OrchardNoteCommitmentTree
    - ✅ Updated migration code to properly populate witness data

- 🔄 **Sapling Note Commitment Trees Implementation** (HIGH PRIORITY)
  - Implement a parser for Sapling note commitment trees
  - Add conversions to appropriate ZeWIF structures
  - Update position tracking for Sapling outputs 
  - Integrate with transaction migration code
  - Test with real-world tree structures from various wallet implementations

#### Future Tasks
- ⏳ **Unified Address Support** (MEDIUM PRIORITY)
  - Add support for unified addresses with multiple receiver types
  - Properly handle diversifier indices

- ⏳ **Transaction Enhancement** (MEDIUM PRIORITY)
  - Extract block height information from transaction metadata
  - Improve transaction filtering and classification

- ⏳ **Key Mapping Improvements** (LOW PRIORITY)
  - Implement robust transparent address derivation from keys and scripts
  - Create a key registry for faster lookups

### High-Priority Tasks

1. **Unified Accounts Migration**
   - **Analysis**:
     - The `wallet.unified_accounts` structure contains critical information about HD account hierarchy
     - It consists of three HashMaps:
       - `address_metadata`: Maps `u256` identifiers to `UnifiedAddressMetadata` objects with diversifier indices and receiver types
       - `full_viewing_keys`: Maps `u256` identifiers to viewing key strings
       - `account_metadata`: Maps `u256` identifiers to `UnifiedAccountMetadata` objects with fingerprint, BIP-44 coin type, account ID, and key ID
     - The key ID in `UnifiedAddressMetadata` links addresses to their accounts in `UnifiedAccountMetadata`

   - **Implementation Plan**:
     - Create multiple accounts based on account_metadata
     - Link addresses to correct accounts using key_id relationships
     - Include diversifier information and viewing keys
     - Update address conversion and transaction assignment logic

## Migration Component Status

### Current Component Development

1. **Viewing Key Migration** (MEDIUM PRIORITY)
   - Current TODO for processing viewing keys during migration
   - Implementation plan:
     - Complete missing viewing key conversion code
     - Preserve viewing key relationships with addresses in ZeWIF format

2. **Memo Data Handling** (MEDIUM PRIORITY)
   - Currently not preserving memo data in transaction outputs
   - Implementation plan:
     - Add proper memo field preservation during in-memory migration
     - Map memo fields to appropriate ZeWIF data structures

### Completed Migration Components

1. **Address-to-Account Mapping** (COMPLETED)
   - ✅ Created Universal Address Identifier System with AddressId enum and AddressRegistry
   - ✅ Implemented transaction assignment to accounts based on address involvement
   - ✅ Added support for unified accounts and proper account assignment
   - ✅ Implemented validation and comprehensive testing

2. **Transaction Data Structure Conversion** (COMPLETED)
   - ✅ Implemented proper transaction data conversion to in-memory ZeWIF structures
   - ✅ Enhanced Transaction structure with version and lock time information
   - ✅ Fixed note position tracking using commitment tree data
   - ✅ Added witness data to transaction outputs

3. **Orchard Data Migration** (COMPLETED)
   - ✅ Implemented conversion of Orchard wallet components
   - ✅ Added proper note commitment tree parsing
   - ✅ Created witness structures for Orchard outputs
   - ✅ Completed position tracking for Orchard actions

4. **Witness Data Migration** (COMPLETED)
   - ✅ Enhanced IncrementalMerkleTree and IncrementalWitness structures
   - ✅ Created OrchardWitness specialized type
   - ✅ Implemented proper witness creation from note commitment trees
   - ✅ Updated transaction migration to populate witness data for outputs

5. **Note Position Preservation** (COMPLETED)
   - ✅ Implemented proper position value extraction from note commitment trees
   - ✅ Added position tracking for Orchard outputs
   - ✅ Added position tracking for Sapling outputs
   - ✅ Preserved correct positional relationships in the ZeWIF format
