# How To Use Attachments with ZeWIF

The ZeWIF data format is build on [Gordian Envelope](https://developer.blockchaincommons.com/envelope/). This provides it with access to many additional features that might not be utilized in the first iteration of the ZeWIF standard but which may be of use in the future, including compression, elision, and encryption.

Another special feature of Envelope is the "attachment", which is fully detailed in [BCR-2023-006](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2023-006-envelope-attachment.md) and summarized here. Attachments _are_ used in ZeWIF, specifically to store content for certain data classes.

## Data Classes in ZeWIF

ZeWIF organizes data into three classes:

* **Class I.** Data that is stored by two or more different wallets and is considered important and up-to-date.
* **Class II.** Data that is stored by less than two wallets or that is considered less important or less up-to-date.
* **Class III.** Data that is considered unimportant, not up-to-date, and is typically not in use by multiple wallets.

Class I data is defined in the ZeWIF spec. Most of this encoding is done through specific CBOR tags, but the specified data structures may incorporate spaces to include attachments, so that all data of a specific type is kept together. Class II data should still be stored as part of a ZeWIF file, but is not specified, so the storage is done through attachments. Class III data is not considered important enough to store, but should still be maintained as part of an attachment containing the entirety of the original wallet data file that is being migrated.

To clarify attachments may be found in three places in a ZeWIF data file:

* **Standard Attachments.** These are defined in the ZeWIF spec as places for attachments as parts of standardized data structures.
* **Freeform Attachments.** These are not defineed in the ZeWIF spec. They are additional attachments that are added to incorporate data not part of the spec.
* **Data File.** This is a special attachment that holds the complete data file of the exporting wallet, so that all class III data can later be recoverd if the classification was in error (or so that data can recovered following an import error that was ignored by the user).

## Envelope Technical Overview

The [Gordian Envelope](https://developer.blockchaincommons.com) is fully defined in an [IETF Internet-Draft](https://datatracker.ietf.org/doc/draft-mcnally-envelope/). Understanding of the Envelope data format is _not_ required to the zmigrate migration tool, even if input is occurring from ZeWIF files or output is occurring to ZeWIF files. The ZeWIF spec fully details what is needed to understand its content; it just happens to defined in a format that is fully compatible with the Envelope spec.

Nonetheless, the following major points are useful as an overview for ZeWIF storage:

* **Envelope Uses CBOR.** Envelope is built using CBOR, a [well-specified](https://cbor.io/) and mature binary data representation format. Every Envelope is not only [legal CBOR](https://datatracker.ietf.org/doc/html/rfc8949), but also [legal dCBOR](https://datatracker.ietf.org/doc/draft-mcnally-deterministic-cbor/), a deterministic version of CBOR. Every Envelope, and therefore every ZeWIF file, can be read using CBOR tools such as [cbor.me](https://cbor.me/). (But don't read ZeWIF files containing private keys in an online site!)
* **Envelope Stores Data in a Merkle-Like Tree.** Envelope is a branching hierarchical structure. Central nodes lead to multiple branches and eventually to leaves. This allows for the organized storage of data. The tree is Merkle-like because branches can be hashed and that hash can be stored in a store to prove that data that lies under it (which may not be relevant for the first iteration of ZeWIF, but which allows for powerful elision and signatures).
* **Envelope is Built on Semantic Triples.** Data is stored in an Envelope as a sematic triple of subject-predicate-object. Each predicate-object pair is called an assertion. A node connects together a subject and zero or more assertions about that subject. 

```mermaid
graph LR
    1("node")
    2["subect"]
    3["assertion"]
    4["predicate"]
    5["object"]
    6["assertion"]
    7["predicate"]
    8["object"]

    1 --> 2
    1 --> 3
    3 --> 4
    3 --> 5
    1 --> 6
    6 --> 7
    6 --> 8
```
* **Envelope Can Be Recursive.** Any subject, predicate, object, or assertion can itself be an Envelope (a semantic triple). This allows for fully recursive structures to improve organization.

```mermaid
graph LR
    1("node")
    2["subect"]
    3["assertion"]
    4["predicate"]
    5["object"]
    6["assertion"]
    7["subject"]
    8["node"]
    9["subject"]
    10["assertion"]
    11["predicate"]
    12["object"]
    1 --> 2
    1 --> 3
    3 --> 4
    3 --> 5
    1 --> 6
    6 --> 7
    6 --> 8
    8 --> 9
    8 --> 10
    10 --> 11
    10 --> 12
```

* 
* 
* **Envelope Can Be Stored as a UR.**
* **Envelopes Can Be Signed.**
* **Envelope Hashes Allow Data to Be Elided.**
 
## Attachments Technical Overview

### Defining `vendor` and `conformsTo`

### Versioning `conformsTo`

## Encoding a Standard Attachment

## Encoding a Freeform Attachment

## Encoding a Data File





```
Deliverable # 2.2: A developer's how-to document describing the use of Envelope attachments for data not included in basic interchange format.
```
