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

## Migration Plan: ZCashd to ZeWIF

### Migration Status and Next Tasks

1. The overall goal is to migrate a `ZcashdWallet` (already read into the structures in `src/zcashd/`) to a `ZewifTop` (using the structures in `src/zewif/`).
2. The main migration function is `src/zcashd/migrate.rs::migrate_to_zewif`.
3. We are focusing on the wallet and account data, with transaction data as a later priority.
4. We are primarily working in the `src/zcashd/migrate.rs` and `src/zewif/` modules.
5. We are focused on moving in-memory representations of the wallet data to the ZeWIF abstractions, we are *not* focused on serialization or file I/O at this time.
6. The next high-priority tasks are:
   - **Unified Accounts Migration**: Convert the `wallet.unified_accounts` structure to ZeWIF format
   - **Note Commitment Trees Migration**: Parse and convert the Orchard note commitment tree

#### Current Tasks
- 🔄 **Note Commitment Trees Migration** (HIGH PRIORITY)
  - Initial implementation for parsing and converting Orchard note commitment tree completed
  - Required improvements:
    - Complete binary tree format parser with proper error handling
    - Add detailed mapping between commitments and their positions in the tree
    - Create proper witness data structures for each output
    - Implement mutable access methods to update transaction outputs with correct positions
    - Test with real-world tree structures from various wallet implementations

#### Future Tasks
- ⏳ **Enhanced Transaction Conversion** (MEDIUM PRIORITY)
  - Improve note position tracking for Sapling and Orchard outputs
  - Add full witness data support for verification
  - Add proper memo field decryption when appropriate keys are available
  - Extract block height information from transaction metadata

- ⏳ **Unified Address Support** (MEDIUM PRIORITY)
  - Add support for unified addresses with multiple receiver types
  - Properly handle diversifier indices

- ⏳ **Key Mapping Improvements** (LOW PRIORITY)
  - Implement robust transparent address derivation from keys and scripts
  - Add proper viewing key support
  - Create a key registry for faster lookups

### High-Priority Zcashd -> ZeWIF Mappings

1. **Unified Accounts Migration**
   - **Analysis**:
     - The `wallet.unified_accounts` structure contains critical information about HD account hierarchy
     - It consists of three HashMaps:
       - `address_metadata`: Maps `u256` identifiers to `UnifiedAddressMetadata` objects containing diversifier indices and receiver types
       - `full_viewing_keys`: Maps `u256` identifiers to viewing key strings
       - `account_metadata`: Maps `u256` identifiers to `UnifiedAccountMetadata` objects containing seed fingerprint, BIP-44 coin type, ZIP-32 account ID, and key ID
     - The key ID in `UnifiedAddressMetadata` links addresses to their accounts in `UnifiedAccountMetadata`

   - **Implementation Plan**:
     - For each entry in `wallet.unified_accounts.account_metadata`:
       1. Create a new `zewif::Account` with a unique ARID
       2. Set the account's `zip32_account_id` to the `UnifiedAccountMetadata.account_id`
       3. Set the account's name to a generated string including account ID (e.g., "Account #0")
     - For each entry in `wallet.unified_accounts.address_metadata`:
       1. Find the account metadata using the address's `key_id`
       2. Link the address to the correct account in the ZeWIF hierarchy
       3. Include diversifier information in the address structure
     - Migrate all relevant viewing keys from `full_viewing_keys` to their corresponding addresses

   - **Required Changes**:
     1. Modify `migrate_to_zewif` function to conditionally process unified accounts
     2. Create a new function `convert_unified_accounts` that:
        - Creates multiple accounts based on `account_metadata`
        - Maps each address to its appropriate account using `key_id` relationships
        - Preserves necessary derivation and diversifier information
     3. Update address conversion functions to check for unified address relationships
     4. Modify existing code that creates a single default account to handle multiple accounts

2. **Note Commitment Trees**
   - **Analysis**:
     - Note commitment trees are critical for transaction validation in ZCash
     - The `ZcashdWallet` contains an `orchard_note_commitment_tree` field of type `OrchardNoteCommitmentTree`
     - `OrchardNoteCommitmentTree` currently just stores unparsed data as a `Data` object
     - ZeWIF provides corresponding structures in:
       - `zewif::IncrementalMerkleTree` with fields for left, right, and parent nodes
       - `zewif::IncrementalWitness<DEPTH, Hash>` for witness data
       - Specialized witness types like `SaplingWitness` and `SproutWitness`
       - Note commitment tree position info in output descriptions

   - **Implementation Plan**:
     1. Parse the raw `orchard_note_commitment_tree.unparsed_data` into a structured format
     2. Convert to appropriate ZeWIF structures:
        - Create `IncrementalMerkleTree` instances
        - Populate witness data
        - Link notes to their positions in the tree
     3. Add the tree data to related transaction outputs in the ZeWIF format

   - **Required Changes**:
     1. Implement a parser for the raw Orchard note commitment tree data
     2. Create a conversion function from ZCashd tree format to ZeWIF format
     3. Update the transaction migration code to include tree and witness data
     4. Ensure that note positions and authentication paths are preserved

### Low-Priority or Zcashd-specific Mappings

1. **KeyPool Information**
   - The `wallet.key_pool` is Zcashd-specific and likely not needed in a generic interchange format.

2. **Client Version Information**
   - The `wallet.client_version` and `wallet.min_version` are Zcashd implementation details.

3. **Block Locator**
   - The `wallet.bestblock` and `wallet.bestblock_nomerkle` are chain-specific sync data.

4. **OrderPosNext**
   - The `wallet.orderposnext` is an internal Zcashd ordering mechanism.

5. **WitnessCacheSize**
   - The `wallet.witnesscachesize` is an implementation detail of Zcashd.

## Unfinished Migration Components in zcashd::migrate

The following items represent unfinished components specifically related to in-memory data migration between wallet representation formats:

### High-Priority Migration Fixes

1. **Address-to-Account Mapping** (HIGH PRIORITY)
   - Current placeholders at several locations using default account
   - Implementation plan (breakdown into subtasks):
     1. **Create a Universal Address Identifier System** (COMPLETED)
        - ✅ Designed a consistent way to identify addresses across different protocols (transparent, sapling, orchard)
        - ✅ Created `AddressId` enum and `AddressRegistry` in `src/zewif/address_id.rs`
        - ✅ Implemented conversion functions and comprehensive unit tests

     2. **Enhance the Unified Accounts Parser**
        - Improve the `convert_unified_accounts` function to properly extract all account-address relationships
        - Create a more robust mapping between ZCash addresses and account IDs

     3. **Fix Transparent Address Assignment**
        - Update the code in `convert_transparent_addresses` to use the account mapping instead of default account
        - Make this function work with both unified and non-unified account scenarios

     4. **Fix Shielded Address Assignment**
        - Similarly update the `convert_sapling_addresses` function to use proper account mapping
        - Ensure shielded addresses are assigned to the correct accounts

     5. **Update Transaction Assignment Logic**
        - Refine how transactions are assigned to accounts based on address involvement
        - Replace the existing placeholder in `extract_transaction_addresses`

     6. **Add Validation and Testing**
        - Add checks to ensure all addresses are mapped to valid accounts
        - Verify correct account-address relationships are maintained during migration

2. **Transaction Data Structure Conversion** (HIGH PRIORITY)
   - Current limitations with representing transaction outputs and actions
   - Implementation plan:
     - Complete proper transaction data conversion to in-memory ZeWIF structures
     - Improve in-memory representation of transaction components
     - Fix note position placeholder values during migration

### Medium-Priority Migration Components

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

3. **Orchard Data Migration** (MEDIUM PRIORITY)
   - Incomplete conversion of Orchard wallet components
   - Implementation plan:
     - Complete conversion of Orchard-specific data structures
     - Fix placeholder code in Orchard address handling

### Low-Priority Migration Improvements

1. **Witness Data Migration** (LOW PRIORITY)
   - Current limitations with witness data conversion
   - Implementation plan:
     - Complete witness data conversion between in-memory formats
     - Properly map witness structures to ZeWIF memory representation

2. **Note Position Preservation** (LOW PRIORITY)
   - Current placeholder position values (Position(0))
   - Implementation plan:
     - Implement proper position value extraction and conversion
     - Preserve correct positional relationships in the ZeWIF format
