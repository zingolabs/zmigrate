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

## Important Basic Types

- `Blob<N>`: A fixed-size byte array (wrapper around `[u8; N]`)
- `Blob32`: Type alias for `Blob<32>`
- `Data`: A variable-size byte array (wrapper around `Vec<u8>`)
- `u256`: A 256-bit unsigned integer (wrapper around `Blob32`), assumes little-endian byte order so reverses on display.

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

1. **Spending Keys Migration** ✅
   - Analysis: Spending keys are essential for wallet functionality but were not previously migrated.
   - Implementation:
     - Enhanced the `SpendingKey` structure to use an explicit enum that properly captures all components:
       - Core cryptographic components (using the appropriate `u256` type):
         - `ask` (spending authorization key) - Enables transaction signing
         - `nsk` (nullifier spending key) - Required for creating nullifiers to prevent double-spending
         - `ovk` (outgoing viewing key) - Allows viewing outgoing transaction details
       - Explicit ZIP-32 HD wallet derivation fields:
         - `depth` - Depth in the HD hierarchy
         - `parent_fingerprint` - Parent key fingerprint for derivation paths
         - `child_index` - Index at current depth in the derivation path
         - `chain_code` - Chain code for key derivation
         - `dk` - Diversifier key for address generation
     - Created a lookup mechanism to find the corresponding `SaplingKey` for a given incoming viewing key
     - Implemented direct field conversion without unnecessary type transformations
     - Used consistent type representations (`u256` for key material) throughout the codebase
     - Added backward compatibility through a Raw key format option
   - Future improvements:
     - Add specialized handling for other key types (Orchard, etc.)
     - Implement proper ZCash protocol serialization standards

2. **Unified Accounts Migration**
   - Analyze: The `wallet.unified_accounts` structure contains critical information about HD account hierarchy.
   - Implementation: Map each unified account to a Zewif account with proper ZIP32 account ID.
   - Required changes: Add code to process `unified_accounts` if present, creating distinct accounts rather than a single default account.

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
