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

1. The goal is to migrate a `ZcashdWallet` (already read into the structures in `src/zcashd/`) to a `ZewifTop` (using the structures in `src/zewif/`).
2. Right now we are are focusing on the wallet and account data, and not the transaction data.
3. Determine whether all high-level data in the `Zcashd` side is being migrated to the `Zewif` side using the `zcashd::migrate::migrate_to_zewif` function.
4. For any mappings not yet present, write best-effort mappings in the `migrate_to_zewif` function.
5. Use `todo!()` for anything deeper than the top level of the wallet and account data.
6. Make sure to document any non-obvious assumptions made.

- Don't generate tests yet.
