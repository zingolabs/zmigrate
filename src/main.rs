use std::env;
use std::path::Path;

use anyhow::{bail, Context, Result};

mod bdb_dump;
pub use bdb_dump::*;
mod blob;
pub use blob::*;
mod data;
pub use data::*;
mod digest_utils;
pub use digest_utils::*;
mod parse_macro;
mod parser;
pub use parser::*;
mod string_utils;
pub use string_utils::*;
mod zcashd;
pub use zcashd::*;

fn main() {
    if let Err(e) = inner_main() {
        println!("---");
        println!("🔴 Error: {}\n", e);
        // Print the error context chain
        for cause in e.chain().skip(1) {
            println!("Caused by: {}", cause);
        }
        std::process::exit(1);
    }
}

fn inner_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Usage: {} <berkeleydb_file>", args[0]);
    }

    let db_dump = BDBDump::from_file(Path::new(&args[1])).context("Parsing BerkeleyDB file")?;

    let zcashd_dump = ZcashdDump::from_bdb_dump(&db_dump).context("Parsing Zcashd dump")?;

    zcashd_dump.print_keyname_summary();

    // println!("---");
    // zcashd_dump.print_keys();

    let (zcashd_wallet, unparsed_keys) =
        ZcashdParser::parse_dump(&zcashd_dump).context("Parsing Zcashd dump")?;

    println!("---");
    println!("{:#?}", zcashd_wallet);

    if unparsed_keys.is_empty() {
        println!("---");
        println!("✅ All keys parsed successfully");
    } else {
        println!("---");
        println!("🛑 Unparsed keys:");
        let mut sorted_keys: Vec<_> = unparsed_keys.into_iter().collect();
        sorted_keys.sort();
        let mut last_keyname: Option<String> = None;
        for key in sorted_keys {
            if let Some(ref last_keyname) = last_keyname {
                if *last_keyname != key.keyname {
                    println!();
                }
            }
            last_keyname = Some(key.keyname.to_string());

            let value = zcashd_dump.value_for_key(&key).unwrap();
            println!("❌ key: {}\n\tvalue: {}", key, value);
        }
    }

    Ok(())
}
