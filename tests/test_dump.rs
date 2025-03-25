use anyhow::Result;
use zmigrate::{zcashd_cmd, zingo_cmd};

use std::path::PathBuf;

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn zcashd_dump(path_elements: &[&str]) -> Result<String> {
    let path = fixtures_dir()
        .join(path_elements.iter().collect::<PathBuf>());

    zcashd_cmd::process_file(&path)
}

fn zingo_dump(path_elements: &[&str]) -> Result<String> {
    let path = fixtures_dir()
        .join(path_elements.iter().collect::<PathBuf>());

    zingo_cmd::process_file(&path)
}

#[test]
fn test_zcashd() {
    let path_elements = ["zcashd", "golden-v5.6.0", "node0_wallet.dat"];
    let output = zcashd_dump(&path_elements).expect("Unable to process file");
    assert!(output.lines().last().unwrap().contains("Success"));
    // println!("{}", output);
}

#[test]
fn test_zingo() {
    let path_elements = ["zingo", "mainnet", "hhcclaltpcckcsslpcnetblr-gf0aaf9347.dat"];
    let output = zingo_dump(&path_elements).expect("Unable to process file");
    assert!(output.lines().last().unwrap().contains("Success"));
    // println!("{}", output);
}
