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

### Next Task
1. The goal is to migrate a `ZcashdWallet` (already read into the structures in `src/zcashd/`) to a `ZewfitWallets` (using the structures in `src/zewif/`).
2. For now, we will focus on the wallet and account data, and not the transaction data.
3. Figure out a mapping from the highest level of the Zcashd wallet to the highest level of the Zewif wallet.
4. Implement the mapping for the wallet and account data.
5. Use `todo!()` for anything deeper than the top level of the wallet and account data.

- Don't generate tests yet.
