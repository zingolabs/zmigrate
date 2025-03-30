#!/bin/bash

set -e

SRC=test/fixtures/zingo
TARGET=dumps/zingo

# cargo run -- zingo ${SRC}/mainnet/hhcclaltpcckcsslpcnetblr-gf0aaf9347.dat > ${TARGET}/mainnet/hhcclaltpcckcsslpcnetblr-gf0aaf9347.txt

WALLETS=(
    regtest/aaaaaaaaaaaaaaaaaaaaaaaa-v26  # version 1 WalletCapabilities
    mainnet/hhcclaltpcckcsslpcnetblr-gf0aaf9347 # version 2 WalletCapabilities
    mainnet/hhcclaltpcckcsslpcnetblr-latest # version 4 WalletCapabilities
    # mainnet/vtfcorfbcbpctcfupmegmwbp-v28 # large
    regtest/aadaalacaadaalacaadaalac-orch-and-sapling
    regtest/aadaalacaadaalacaadaalac-orch-only
    regtest/hmvasmuvwmssvichcarbpoct-v27
    testnet/G93738061a
    testnet/Gab72a38b
    testnet/cbbhrwiilgbrababsshsmtpr-latest
    testnet/glory_goddess
    testnet/latest
    testnet/v26
    # testnet/v27 # large
    testnet/v28
)


# Process each wallet file
for wallet in "${WALLETS[@]}"; do
    FILE=${wallet}
    IN_FILE=${FILE}.dat
    OUT_FILE=${FILE}.txt
    # echo "Processing ${IN_FILE}..."
    echo "cargo run -- zingo ${SRC}/${IN_FILE} > ${TARGET}/${OUT_FILE}"
    cargo run -- zingo ${SRC}/${IN_FILE} > ${TARGET}/${OUT_FILE}
done
