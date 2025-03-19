# CLAUDE.md - ZMigrate Guidelines

## Introduction

The `zmigrate` crate provides tooling to migrate wallets from various ZCash implementations (zcashd, zingo, etc.) to the common Zewif interchange format. Eventually the wallet-specific parts will be moved to separate crates, which will provide front-end and back-end implementations for each wallet type. The ZeWIF file format itself, which will be directly supported by the `zmigrate` crate will be based on Gordian Envelope, with wallet-specific and esoteric data stored in attachments. The `zmigrate` command line tool will to and from Zewif files, and will be able to convert between wallet types.

### Purpose of ZMigrate

The `zmigrate` library serves several key purposes:
1. **Wallet Migration** - Enables users to convert their existing wallets from specific implementations to a universal format
2. **Wallet Interoperability** - Allows wallet data to be moved between different ZCash wallet implementations
3. **Data Preservation** - Ensures no critical wallet data is lost during transfers between implementations
4. **Key Recovery** - Facilitates recovery of keys and addresses from various wallet formats

### Zewif Interchange Format

Zewif (ZCash Wallet Interchange Format) is a specification designed to:
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
- `u256`: A 256-bit unsigned integer (wrapper around `Blob32`), assumes little-endian byte order so reverses on display, usually used for cryptographic values.

### Coding Notes

- Do *not* use the attachments feature yet. It will be used for wallet-specific data that is not part of the core Zewif interchange format, but which will be preserved by it.
- Don't generate tests yet.

## Migration Plan: ZCashd to Zewif

### Next Task

1. The goal is to migrate a `ZcashdWallet` (already read into the structures in `src/zcashd/`) to a `ZewifTop` (using the structures in `src/zewif/`).
2. The main migration function is `src/zcashd/migrate.rs::migrate_to_zewif`.
3. Right now we are focusing on the wallet and account data, and not the transaction data.
4. DELIVERABLE: First draft of the migration of item 1 below: `wallet.sapling_keys`.

### High-Priority Zcashd -> Zewif Mappings

1. **Unified Accounts Migration**
   - Analyze: The `wallet.unified_accounts` structure contains critical information about HD account hierarchy.
   - Implementation: Map each unified account to a Zewif account with proper ZIP32 account ID.
   - Required changes: Add code to process `unified_accounts` if present, creating distinct accounts rather than a single default account.

2. **Note Commitment Trees**
   - Analyze: Note commitment trees are important for transaction validation.
   - Implementation: Migrate the Orchard note commitment tree to appropriate Zewif structures.

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
