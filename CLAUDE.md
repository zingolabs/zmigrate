# CLAUDE.md - ZMigrate Guidelines

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

### Coding Notes

- Do *not* use the attachments feature yet. It will be used for wallet-specific data that is not part of the core Zewif interchange format, but which will be preserved by it.
- Don't generate tests yet.

## Migration Plan: ZCashd to Zewif

### Next Task

1. The goal is to migrate a `ZcashdWallet` (already read into the structures in `src/zcashd/`) to a `ZewifTop` (using the structures in `src/zewif/`).
2. Right now we are are focusing on the wallet and account data, and not the transaction data.
3. Determine whether all high-level data in the `Zcashd` side is being migrated to the `Zewif` side using the `zcashd::migrate::migrate_to_zewif` function.
4. For any mappings not yet present, perform a triage. First, decide whether the mapping is high-priority or low-priority.
   1. If high-priority, document it in the `High-Priority Zcashd -> Zewif Mappings` section below. For each high-priority mapping, write your analysis of what is required to migrate the data, and what assumptions are being made.
   2. If low-priority or Zcashd-specific, document it in the `Low-Priority or Zcashd-specific Mappings` section below.

### High-Priority Zcashd -> Zewif Mappings

1. **Unified Accounts Migration**
   - Analyze: The `wallet.unified_accounts` structure contains critical information about HD account hierarchy.
   - Implementation: Map each unified account to a Zewif account with proper ZIP32 account ID.
   - Required changes: Add code to process `unified_accounts` if present, creating distinct accounts rather than a single default account.

2. **Spending Keys Migration**
   - Analyze: Spending keys are essential for wallet functionality but are not currently migrated.
   - Implementation: Extract spending keys from `wallet.sapling_keys` and other key structures, and add them to the appropriate Zewif address structures.

3. **Note Commitment Trees**
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
