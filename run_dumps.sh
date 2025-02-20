#!/bin/bash

SRC=dat_files
TARGET=dumps

VERSION=sprout
SRC_DIR=${SRC}/${VERSION}
TARGET_DIR=${TARGET}/${VERSION}
cargo run -- ${SRC_DIR}/node0_wallet.dat > ${TARGET_DIR}/node0_wallet.txt
cargo run -- ${SRC_DIR}/node1_wallet.dat > ${TARGET_DIR}/node1_wallet.txt
cargo run -- ${SRC_DIR}/node2_wallet.dat > ${TARGET_DIR}/node2_wallet.txt
cargo run -- ${SRC_DIR}/node3_wallet.dat > ${TARGET_DIR}/node3_wallet.txt

VERSION=golden-v5.6.0
SRC_DIR=${SRC}/${VERSION}
TARGET_DIR=${TARGET}/${VERSION}
cargo run -- ${SRC_DIR}/node0_wallet.dat > ${TARGET_DIR}/node0_wallet.txt
cargo run -- ${SRC_DIR}/node1_wallet.dat > ${TARGET_DIR}/node1_wallet.txt
cargo run -- ${SRC_DIR}/node2_wallet.dat > ${TARGET_DIR}/node2_wallet.txt
cargo run -- ${SRC_DIR}/node3_wallet.dat > ${TARGET_DIR}/node3_wallet.txt

VERSION=tarnished-v5.6.0
SRC_DIR=${SRC}/${VERSION}
TARGET_DIR=${TARGET}/${VERSION}
cargo run -- ${SRC_DIR}/node0_wallet.dat > ${TARGET_DIR}/node0_wallet.txt
cargo run -- ${SRC_DIR}/node1_wallet.dat > ${TARGET_DIR}/node1_wallet.txt
cargo run -- ${SRC_DIR}/node2_wallet.dat > ${TARGET_DIR}/node2_wallet.txt
cargo run -- ${SRC_DIR}/node3_wallet.dat > ${TARGET_DIR}/node3_wallet.txt

SRC_DIR=${SRC}
TARGET_DIR=${TARGET}
cargo run -- ${SRC_DIR}/wallet0.dat > ${TARGET_DIR}/wallet0.txt
cargo run -- ${SRC_DIR}/wallet1.dat > ${TARGET_DIR}/wallet1.txt
cargo run -- ${SRC_DIR}/wallet2.dat > ${TARGET_DIR}/wallet2.txt
cargo run -- ${SRC_DIR}/wallet3.dat > ${TARGET_DIR}/wallet3.txt
cargo run -- ${SRC_DIR}/wallet4.dat > ${TARGET_DIR}/wallet4.txt
cargo run -- ${SRC_DIR}/wallet5.dat > ${TARGET_DIR}/wallet5.txt
cargo run -- ${SRC_DIR}/wallet6.dat > ${TARGET_DIR}/wallet6.txt
cargo run -- ${SRC_DIR}/wallet7.dat > ${TARGET_DIR}/wallet7.txt
