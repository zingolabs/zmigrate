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

## Migration Plan: ZCashd to Zewif

### Implementation Strategy
1. Create `src/zcashd/migrate.rs` module with function `migrate_to_zewif(wallet: &ZcashdWallet) -> ZewifWallets`
2. Design mapping approach using conversion traits:
   - Implement `From<ZcashdWallet> for WalletDB`
   - Create mapping functions for complex nested types
3. Data transformation steps:
   - Generate appropriate ARIDs for wallet and accounts
   - Map seed material from mnemonic phrase
   - Convert transparent and shielded addresses
   - Transform transactions with TXID-based lookups
   - Map wallet-specific metadata to appropriate attachments
4. Add comprehensive context-rich error handling
5. Write unit tests with sample wallet data
