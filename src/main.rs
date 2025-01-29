use std::env;
use std::path::Path;

use anyhow::{Result, bail, Context};

mod bdb_dump;
pub use bdb_dump::BDBDump;
mod zcashd;
pub use zcashd::*;
mod blob;
pub use blob::*;
mod data;
pub use data::*;
mod digest;
pub use digest::*;
mod parser;
pub use parser::*;
mod seconds_since_epoch;
pub use seconds_since_epoch::*;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        bail!("Usage: {} <berkeleydb_file>", args[0]);
    }

    let db_dump = BDBDump::from_file(Path::new(&args[1]))
        .context("Failed to parse BerkeleyDB file")?;

    let zcashd_dump = ZcashdDump::from_bdb_dump(&db_dump)
        .context("Failed to parse Zcashd dump")?;

    zcashd_dump.print_keyname_summary();

    // println!("---");
    // zcashd_dump.print_keys();

    let zcashd_wallet = ZcashdParser::parse_dump(&zcashd_dump)
        .context("Failed to parse Zcashd dump")?;

    println!("---");
    println!("{:#?}", zcashd_wallet);

    Ok(())
}
