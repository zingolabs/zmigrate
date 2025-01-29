use std::env;
use std::path::Path;

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

fn main() {
    fn fail(msg: &str) -> ! {
        eprintln!("{}", msg);
        std::process::exit(1);
    }

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        fail("Usage: {} <berkeleydb_file>");
    }

    let db_dump = BDBDump::from_file(Path::new(&args[1])).unwrap_or_else(|e| {
        fail(&format!("Failed to parse BerkeleyDB file: {}", e));
    });

    let zcashd_dump = ZcashdDump::from_berkeley_dump(&db_dump).unwrap_or_else(|e| {
        fail(&format!("Failed to parse Zcashd dump: {}", e));
    });

    zcashd_dump.print_keyname_summary();

    // println!("---");
    // zcashd_dump.print_keys();

    let zcashd_wallet = ZcashdParser::parse_dump(&zcashd_dump).unwrap_or_else(|e| {
        fail(&format!("Failed to parse Zcashd dump: {}", e));
    });

    println!("---");
    println!("{:#?}", zcashd_wallet);
}
