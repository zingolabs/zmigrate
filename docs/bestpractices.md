# Best Practices for Importing & Exporting Wallet Data

The ZeWIF format is a specification for the import and export of Zcash wallet data, enabling interoperability among walls, and creating freedom for users. However, the ZeWIF format also requires work from individual developers who want to interop their data, whether they are using the ZeWIF format or not.

```mermaid
graph TD
    A[(zcashd)] --> B[front-ends]
    D[(zecwallet)] --> B
    C[(...)] --> B
    E[(...)] --> B
    H[(zewif)] --> G
    B --> G[zmigrate]
    G --> I[(zewif)]
    G --> M[back-ends]
    M --> J[(zecwallet)]
    M --> K[(...)]
    M --> L[(...)]
```

As the above diagram shows, the zmigrate Rust crate lies at the center of the ZeWIF system. It creates in-memory representations of data from a variety of inputs and can  output that abstracted data in a numbers of forms. Obviously, it can accept input from ZeWIF and it can output to ZeWIF. However, that's just part of the process. Individual developers can also choose to use creat front ends that interface their own wallet data to zmigrate and back ends that export the data from zmigrate to their own wallet.

The following best practices offer suggestions for those front-end and back-end wallet developers, to ensure that their data remains not just maximally interoperable, but also maximally accessible, both now and in the far future.

## The Core Format

***Use Defined CBOR Tags.*** If there is a ZeWIF-defined CBOR tag for a piece of data that is being migrated, that CBOR tag should be used, even if it requires converting the data type as part of the migration.

* _Example:_ [an example of a CBOR and something from a wallet that should be stored in that format, especially if the wallet stores it with a different data type]

***Break Apart Composite Data.*** If a single datum in a wallet contains several individual keys and values, they should be separated out before migrating them, even if they're related.

* _Example:_ `zcashd`'s CKeyMetaData contains a seed fingerprint (uint256), a creation date (UNIX timestamp), and an HD/ZIP-32 Keypath (string). Those datums should each be individually stored when migrated.

## Key Migration

***Store Existing Assets As They Are, Usually.*** In the vast majority of cases, the migration process should happen without making any changes on the blockchain. This is not the time to do other clean-up, except in a few important cases. You want to preserve the data being imported as it is, because it was theoretically in a known, working state.

* _Example:_ A user has lots of Zcash dust that can't be used effectively. Nonetheless, the keys controlling that dust should be converted over. The new wallet can decide if it wants to do anything with the issue.

***Resolve Bugs In Asset Control If Possible.*** If your wallet did something out of spec with the larger Zcash community and it affects the control of assets, this is important to resolve before exporting your data, because future wallets will not know about the variance from the specification and it could cause lost of funds. Spec variance is mostly likely to be a variance in how keys are derived from a seed or a master key, but there might be other issues. In these cases, move funds off of the out-of-spec keys or seeds (or whatever) before migrating the wallet file.

* _Example:_ `zecwallet-cli 1.0` incorrectly derived HD wallet keys after the first key, affecting both `t` and `z` addresses. Funds on addresses after the first should be moved prior to the migration of a `zecwallet-cli 1.o` wallet as future wallets won't know about these incorrectly derived keys, and thus will not be able to access the funds without knowing specifically how to derive them.

***Migrate Sprout-Keyed Assets If Desired.***


## Calculated Data

***Store All Transactions Information.***

***Store Almost All Witness Trees.***

***Dump Incorrect Witness Trees.***

## Attachments

***Store Undefined Data with Attachments.***

***Drop Wallet-Specific Configuration.***

1. Can drop wallet specific stuff, like minversion
***Document Attachments Whenever Possible.***

ways to define things better than bstring [blob]? 
[+metadata, date, etc.]

***Store the Entire Data Set.***

## Encryption

***Decrypt All Data.***

***Re-Encrypt for Insecure Transmission.***

***Re-Encrypt for Storage.***

can encrypt with encryption or SSKR [can be done with Envelope tool, not in spec]

## Elision & Compression

***Compress as Necessary.***

***Elide Thoughtfully.***

Be clear that anything can be wrapped to elide, compress [e.g. large string, not random numbers] if space tight

may want to differentiate between what would give to an auditor and to a wallet: might do as multiple attachments [privacy-breaking]

## Reports

***Report All Failures.***

```
Deliverable # 3.4: A best practices document on importing & exporting data.
```
