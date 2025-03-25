use anyhow::{Result, bail};
use zmigrate::{zcashd_cmd, zingo_cmd};

use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn dump_wallet(path_elements: &[&str]) -> Result<String> {
    let path = fixtures_dir().join(path_elements.iter().collect::<PathBuf>());
    if path_elements[0] == "zcashd" {
        zcashd_cmd::dump_wallet(&path)
    } else if path_elements[0] == "zingo" {
        zingo_cmd::dump_wallet(&path)
    } else {
        bail!("Unknown command: {}", path_elements[0]);
    }
}

fn test_dump(path_elements: &[&str]) {
    let output = dump_wallet(path_elements)
        .unwrap_or_else(|_| panic!("Unable to process file: {:?}", path_elements));
    assert!(output.lines().last().unwrap().contains("Success"));
    // println!("{}", output);
}

#[test]
fn test_zcashd() {
    let paths = vec![
        vec!["zcashd", "golden-v5.6.0", "node0_wallet.dat"],
        vec!["zcashd", "golden-v5.6.0", "node1_wallet.dat"],
        vec!["zcashd", "golden-v5.6.0", "node2_wallet.dat"],
        vec!["zcashd", "golden-v5.6.0", "node3_wallet.dat"],

        vec!["zcashd", "tarnished-v5.6.0", "node0_wallet.dat"],
        vec!["zcashd", "tarnished-v5.6.0", "node1_wallet.dat"],
        vec!["zcashd", "tarnished-v5.6.0", "node2_wallet.dat"],
        vec!["zcashd", "tarnished-v5.6.0", "node3_wallet.dat"],

        vec!["zcashd", "sprout", "node0_wallet.dat"],
        vec!["zcashd", "sprout", "node1_wallet.dat"],
        vec!["zcashd", "sprout", "node2_wallet.dat"],
        vec!["zcashd", "sprout", "node3_wallet.dat"],

        vec!["zcashd", "wallet0.dat"],
        vec!["zcashd", "wallet1.dat"],
        vec!["zcashd", "wallet2.dat"],
        vec!["zcashd", "wallet3.dat"],
        vec!["zcashd", "wallet4.dat"],
        vec!["zcashd", "wallet5.dat"],
        vec!["zcashd", "wallet6.dat"],
        vec!["zcashd", "wallet7.dat"],
    ];
    for path in &paths {
        test_dump(path);
    }
}

#[test]
fn test_zingo() {
    let paths = vec![
        vec!["zingo", "mainnet", "hhcclaltpcckcsslpcnetblr-gf0aaf9347.dat"],
        vec!["zingo", "mainnet", "hhcclaltpcckcsslpcnetblr-latest.dat"],
        // vec!["zingo", "mainnet", "vtfcorfbcbpctcfupmegmwbp-v28.dat"], // long

        vec!["zingo", "regtest", "hmvasmuvwmssvichcarbpoct-v27.dat"],
        vec!["zingo", "regtest", "aadaalacaadaalacaadaalac-orch-only.dat"],
        vec!["zingo", "regtest", "aadaalacaadaalacaadaalac-orch-and-sapling.dat"],
        vec!["zingo", "regtest", "aaaaaaaaaaaaaaaaaaaaaaaa-v26.dat"],

        vec!["zingo", "testnet", "cbbhrwiilgbrababsshsmtpr-latest.dat"],
        vec!["zingo", "testnet", "G93738061a.dat"],
        vec!["zingo", "testnet", "Gab72a38b.dat"],
        vec!["zingo", "testnet", "glory_goddess.dat"],
        vec!["zingo", "testnet", "latest.dat"],
        vec!["zingo", "testnet", "v26.dat"],
        vec!["zingo", "testnet", "v28.dat"],
        // vec!["zingo", "testnet", "v27.dat"], // long
    ];
    for path in &paths {
        test_dump(path);
    }
}
