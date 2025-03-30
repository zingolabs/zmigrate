# ZCash Wallet Export/Import Format (ZeWIF) Guidelines

## Project Overview

This project provides a universal format for migrating wallet data between different ZCash wallet implementations:

- `zewif`: Core library defining the ZeWIF interchange format
- `zmigrate`: Command line tool for wallet migrations
- `zewif-zcashd`: ZCashd-specific integration for migration
- `zewif-zingo`: Zingo-specific integration for migration (future)

🚨 NOTE: Only make changes to these four crates and no others in the workspace.

The ZeWIF format is designed for **data interchange**, not operational wallet functionality:

- **Purpose**: Enable migration between different ZCash wallet implementations
- **Focus**: Data preservation and format conversion
- **Approach**: Uses optional fields to handle differences between wallet implementations

## Development Environment

### Build/Test Commands

```bash
# Build a specific crate
cargo build -p <crate_name>

# Run the zmigrate tool
cargo run -p zmigrate -- zcashd zmigrate/tests/fixtures/zcashd/wallet0.dat

# Run tests
cargo test -p <crate_name>
cargo test -p <crate_name> --test <test_name>

# Check code quality
cargo clippy -p <crate_name> -- -D warnings

# Build documentation
cargo doc -p zewif --no-deps --target-dir zewif/cargo-docs
```

### Development Guidelines

- **Use accessors, not pub fields** - All struct fields should be private with accessor methods
- **Production quality** - Write code as you would for a real-world implementation
- **Proper error handling** - Use `Result<T>` with `anyhow::Context` for all functions that can fail
- **Clean code** - Fix all compiler errors and Clippy lints

### Testing

- Add tests to `zmigrate/tests` directory
- Test fixtures are stored in `zmigrate/tests/fixtures/`
- Don't mark tasks as complete until all tests pass

## Core Reference: ZeWIF Types and Concepts

### Key Data Types

| Type          | Description                                               |
| ------------- | --------------------------------------------------------- |
| `ZewifTop`    | Top-level container for wallet data                       |
| `ZewifWallet` | Complete wallet representation                            |
| `Blob<N>`     | Fixed-size byte array (wrapper around `[u8; N]`)          |
| `Data`        | Variable-size byte array (wrapper around `Vec<u8>`)       |
| `u256`        | 256-bit unsigned integer, little-endian byte order        |
| `u252`        | 252-bit unsigned integer for Orchard note commitments     |
| `u160`        | 160-bit unsigned integer for transparent addresses        |
| `TxId`        | Transaction ID (32 bytes representing a transaction hash) |

### Useful Macros

| Macro | Description |
|-------|-------------|
| `blob!` | Creates a new type wrapping a fixed-size byte array (`Blob<N>`) with common methods and trait implementations (Parse, Debug, etc.) |
| `data!` | Creates a new type wrapping a variable-size byte array (`Data`) with common methods and trait implementations |
| `string!` | Creates a new type wrapping a String with common methods and trait implementations for string handling |
| `mod_use!` | Simplifies module imports by both declaring a module and re-exporting its contents in a single statement |
| `impl_attachable!` | Automatically implements the Attachable trait for a type that contains an `attachments` field |
| `parse!` | Provides context-aware parsing for types that implement the `Parse` trait with error messages for various parser operations |

### Other Important Types

| Type  | Description |
| ----- | ----------- |
| `bc_components::ARID` | Apparently Random Identifier (ARID), used where structs themselves aren't simple or unique enough to be uniquely identifying. |

### Reference Materials

- Original `zcashd` code: `reference-code/zcash-master` directory
- Transaction assignment details: `zewif-zcashd/docs/TransactionAssignment.md`
- Key preservation strategy: `zewif-zcashd/docs/KeyPreservation.md`

## Current Status and Roadmap

### 🟢 COMPLETED: ZCashd to ZeWIF Migration

We have successfully implemented the complete migration pathway from ZCashd wallet.dat files to the ZeWIF format. All critical data is now properly preserved:

- **Address Migration**: All address types (transparent, sapling, unified) with metadata
- **Transaction Migration**: Complete transaction data with inputs, outputs, and witness data
- **Key Preservation**: All key materials are properly extracted and preserved
- **Metadata Handling**: Purpose strings, names, and other metadata are preserved
- **Unified Address Support**: Diversifier indices and receiver types preserved
- **Encrypted Memos**: Ciphertexts preserved without decryption (by design)
- **Account Structure**: Multi-account wallet structure correctly maintained

#### Key Design Insights

- **Unified Address Handling**: Unified addresses are not stored directly in wallet.dat. Instead, we preserve the metadata (diversifier indices, receiver types, key references) needed to derive them.
- **Encrypted Memos**: Memos are kept in encrypted form, as decryption requires keys that may only be available to the receiving wallet.
- **Block Heights**: While we preserve block hashes, deriving block heights would require blockchain access.

### 🟠 NEXT STEPS: Documentation

Our primary focus now is to add comprehensive documentation to the `zewif` crate to make it easily adoptable by Zcash wallet engineers.

#### Documentation Plan

1. **Core Data Structure Documentation**:

*Prioritized Types*:

| Priority | Type | Current Status | Target Completeness | Progress |
|----------|------|---------------|-------------------|----------|
| 1 | `ZewifTop` | Basic | Complete with examples | Not started |
| 1 | `ZewifWallet` | Basic | Complete with examples | Not started |
| 1 | `Account` | Basic | Complete with examples | Not started |
| 1 | `Address` | Minimal | Complete with examples | Not started |
| 1 | `Transaction` | Minimal | Complete with examples | Not started |
| 1 | `UnifiedAddress` | Good | Nearly complete | Not started |
| 2 | `TransparentAddress` | Minimal | Complete | Not started |
| 2 | `ShieldedAddress` | Minimal | Complete | Not started |
| 2 | `TxIn`/`TxOut` | Minimal | Complete | Not started |
| 2 | Core helpers | Minimal | Complete | ✅ Completed: `Blob`, `Data` only |
| 2 | Numeric types (`u256`, `u252`, `u160`) | Minimal | Complete | ✅ Completed |
| 2 | Parsing infrastructure | Minimal | Complete | ✅ Completed: `parse!` macro |
| 3 | Proof types | Minimal | Complete | Not started |
| 3 | Witness types | Minimal | Complete | Not started |
| 3 | Other helpers | Minimal | Complete | Not started |

*Documentation Template for Each Type*:

🚨 NOTE: DO NOT be pedantic about including every section of documentation in the template below. Only include the most important and useful sections. In particular, don't provide examples unless they are truly useful enough to warrant the time it will take to develop, test, and maintain them.

🚨 NOTE: Be sure to document public free functions, and put a one-line comment in front of each trait impl.

```rust
//! Module-level documentation uses the `//!` syntax and should describe the overall
//! purpose of the module and key types/functionality it provides.

/// A comprehensive description of what this type represents and its purpose
/// in the ZeWIF data model. The first line should be a short summary, followed
/// by a more detailed explanation.
///
/// This type can be used to [do something], and is typically part of the
/// [`ZewifWallet`] structure.
///
/// # Zcash Concept Relation
/// How this type relates to Zcash protocol concepts.
///
/// # Data Preservation
/// What data from wallet.dat files is preserved in this type.
///
/// # Examples
/// ```
/// use zewif::{ExampleType, Network};
///
/// // Example code showing how to create and use this type
/// let example = ExampleType::new();
/// ```
#[derive(Debug, Clone)]
pub struct ExampleType {
    // Fields should have doc comments if they're public or important to understand
    field: String,
}

impl ExampleType {
    /// Creates a new instance of `ExampleType`.
    ///
    /// The first line should be a concise summary of what the method does.
    /// Additional details can follow in subsequent paragraphs.
    ///
    /// # Examples
    /// ```
    /// use zewif::ExampleType;
    ///
    /// let example = ExampleType::new();
    /// ```
    pub fn new() -> Self {
        Self { field: String::new() }
    }

    /// A brief description of what this method does.
    ///
    /// # Arguments
    /// * `arg` - Description of the argument
    ///
    /// # Returns
    /// Description of the return value. Only needed if the return value isn't obvious
    /// or needs explanation.
    ///
    /// # Errors
    /// This section is only needed for methods that return `Result`:
    /// * When `arg` is invalid - returns `ErrorType::InvalidArg`
    /// * When operation fails - returns `ErrorType::OperationFailed`
    ///
    /// # Panics
    /// Only include this section if the method can panic:
    /// * Panics if `some_condition` is not met
    ///
    /// # Examples
    /// ```
    /// use zewif::{ExampleType, SomeType};
    ///
    /// let example = ExampleType::new();
    /// let arg = SomeType::new();
    /// let result = example.example_method(&arg)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// Examples that should compile but not run during tests can use:
    /// ```no_run
    /// # use zewif::{ExampleType, SomeType};
    /// # let example = ExampleType::new();
    /// # let arg = SomeType::new();
    /// // Code that would fail in a test environment
    /// example.example_method(&arg)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn example_method(&self, arg: &SomeType) -> Result<ReturnType, Error> {
        // Implementation here
    }
}
```

2. **Macro Documentation**:
   - Document all macros with detailed explanations and examples
   - Clearly explain what the macros generate and when to use them
   - Include examples of macro usage for all core types

3. **Data Model Overview**:
   - Create a comprehensive module-level documentation in `lib.rs`
   - Explain the overall structure and relationships between core types
   - Document the conceptual organization of the ZeWIF format
   - Include diagrams illustrating key type relationships

4. **Implementation Examples**:
   - Create concise, self-contained examples for common operations:
     - Creating and populating a new wallet
     - Adding addresses to accounts
     - Handling transactions
     - Working with unified addresses
     - Serializing/deserializing ZeWIF data

5. **Cross-Component Documentation**:
   - Document how `zewif` interacts with other crates:
     - `zmigrate` for command-line operations
     - `zewif-zcashd` for ZCashd migration
     - Future integration with `zewif-zingo`
     - Potential integration with Gordian Envelope

#### Implementation Strategy

1. **First Pass**: Focus on core types (`ZewifTop`, `ZewifWallet`, `Account`, `Address`, `Transaction`)
   - Document all public methods and fields
   - Add detailed type-level documentation
   - Create basic examples

2. **Second Pass**: Address specialized types and their relationships
   - Document address types (transparent, shielded, unified)
   - Document transaction components
   - Document key-related types

3. **Third Pass**: Complete documentation for all remaining types
   - Document helper types and utility functions
   - Document proof and witness types
   - Ensure consistent cross-references

4. **Final Pass**: Module-level documentation and integration examples
   - Document module structure in `lib.rs`
   - Create comprehensive usage examples
   - Verify documentation completeness and accuracy

#### Documentation Quality Criteria

- **Comprehensive**: All public API elements have documentation
- **Contextual**: Documentation explains both "what" and "why"
- **Practical**: Examples demonstrate real-world usage
- **Consistent**: Uniform style and detail level across the codebase
- **Accessible**: Explanations suitable for Zcash engineers not familiar with Rust, and Rust engineers not familiar with Zcash
- **Searchable**: Proper cross-references and keyword usage
- **Validated**: Examples compile and work correctly

#### Documentation Testing and Verification

- **Test Documentation Examples:**
  ```bash
  # Run documentation tests to verify examples compile and work
  cargo test --doc -p zewif

  # Build documentation to review rendering
  cargo doc -p zewif --no-deps --target-dir zewif/cargo-docs
  ```

- **Doc Example Guidelines:**
  - **Always test examples** before considering documentation complete
  - **Hide imports** with `#` prefix: `# use zewif::Type;`
  - **Fix visibility issues** by using `pub(crate)` types in test setup when needed
  - **Handle errors properly** in examples that return `Result`
  - **Mark non-running examples** with `no_run` when they can't be tested automatically
  - **Use correct path references** - for doc tests, use `zewif::` not `crate::`
  - **Demonstrate `parse!` macro** usage for types that support parsing

  Correct example format:
  ```rust
  /// # Examples
  /// ```
  /// # use zewif::{Data, SomeOtherType};
  /// # use anyhow::Result;
  /// let data = Data::new();
  ///
  /// // Example implementation
  /// # Ok::<(), anyhow::Error>(()) // Handle Result return
  /// ```
  ```

- **Common Doc Test Fixes:**
  - Use `zewif::` instead of `crate::` in doc tests
  - Add `#![feature(doc_cfg)]` to lib.rs if needed for conditional compilation
  - For failing Result handling, add `# Ok::<(), anyhow::Error>(())` at end
  - For examples that access private fields, either refactor or mark `no_run`
  - For examples requiring external resources, use `no_run`

- **Final Verification:**
  - Ensure all doc tests pass with `cargo test --doc -p zewif`
  - Review generated documentation for completeness and accuracy
  - Ensure all public items have documentation
  - Have another team member review for clarity and completeness

#### Completed Documentation

**Core Helper Types**:
- ✅ `Blob<N>`: Added comprehensive type, method, and Parse trait implementation documentation with examples
- ✅ `Data`: Added comprehensive type, method, and Parse trait implementation documentation with examples
- ✅ `parse!` macro: Added detailed documentation with multiple usage examples
- ✅ `CompactSize`: Documented Bitcoin/Zcash variable-length integer encoding with parsing examples
- ✅ `AddressId`: Documented universal identifier for addresses across different protocols

**Numeric Types**:
- ✅ `u256`: Documented with detailed Zcash protocol context, examples, and parsing documentation
- ✅ `u252`: Documented with Orchard protocol-specific details and validation examples
- ✅ `u160`: Documented with transparent address usage and P2PKH/P2SH context

**Transaction Components**:
- ✅ `TxId`: Documented transaction identifier with details about byte order conventions and examples
- ✅ `Amount`: Documented Zcash currency amount type with ZEC/zat conversion information
- ✅ `Script`: Documented Bitcoin-style script container for transaction inputs/outputs

**Shielded Protocol Structures**:
- ✅ `Position`: Documented note commitment tree position with details about merkle witnesses
- ✅ `ReceiverType`: Documented types of receivers in Unified Addresses
- ✅ `Anchor`: Documented commitment tree roots used for shielded transaction validation
- ✅ `sapling::SaplingExpandedSpendingKey`: Documented core cryptographic components of Sapling spending keys
- ✅ `sapling::SaplingExtendedSpendingKey`: Documented hierarchical deterministic Sapling spending keys with ZIP-32 info
- ✅ `sapling::SaplingFullViewingKey`: Documented viewing keys that can detect both incoming and outgoing transactions
- ✅ `sapling::SaplingOutputDescription`: Documented received note descriptions in Sapling shielded transactions
- ✅ `sapling::SaplingWitness`: Documented cryptographic witnesses for note commitment inclusion proofs
- ✅ `IncrementalMerkleTree`: Documented efficient partial Merkle tree implementation for note commitments
- ✅ `IncrementalWitness`: Documented authentication path generator for proving note inclusion
- ✅ `SproutWitness`: Documented Sprout-specific witness type for legacy shielded transactions
- ✅ `JoinSplitDescription`: Documented Sprout mechanism for transparent-shielded value conversion

**Blockchain Metadata**:
- ✅ `BlockHeight`: Documented block positioning in the blockchain with arithmetic operations
- ✅ `SecondsSinceEpoch`: Documented Unix timestamp implementation for transaction timestamps

**Network and Consensus**:
- ✅ `Network`: Documented Zcash network environments (mainnet, testnet, regtest)
- ✅ `BranchId`: Documented Zcash network upgrades and consensus branches

**Utility Functions**:
- ✅ `string_utils`: Documented formatting functions for Zcash amounts and large numbers

**Utility Macros**:
- ✅ `blob!`: Documented macro for creating fixed-size binary data wrappers
- ✅ `data!`: Documented macro for creating variable-size binary data wrappers
- ✅ `string!`: Documented macro for creating string wrapper types

**Lessons Learned**:
1. Doc test examples use `zewif::Type` imports (not `crate::Type`)
2. The `parse!` macro should be demonstrated for all parseable types
3. Examples need proper setup with hidden imports and error handling
4. All examples must be verified by running `cargo test --doc`
5. Proper cross-referencing between related types improves documentation quality
6. Type documentation should explain both technical details and Zcash protocol context
7. For specialized types (like `u252`), explain their unique constraints and protocol relationship

### 🔵 FUTURE ENHANCEMENTS

#### Zewif serialization to Gordian Envelope

- Implement serialization of ZeWIF data structures to Gordian Envelope format. This is the actual "Zcash Wallet Interchange Format" (ZWIF) that will be used for wallet data at rest.

#### Address and Key Relationship Enhancement

- HD path data preservation for transparent addresses
- Key origin information extraction and storage
- Improved derivation path tracing and verification

#### Unified Address String Derivation

- Implement full unified address string derivation from preserved metadata
- Use the `zcash_address` crate to derive actual unified address strings
- Extract necessary key material for each receiver type
- Implement cryptographic derivation logic

#### Zingo Wallet Support

- Implement Zingo wallet migration pathway
- Handle Zingo-specific data formats and metadata
- Ensure proper integration with the existing ZeWIF format
- Create comprehensive tests for Zingo wallet fixtures
