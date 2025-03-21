mod bdb_dump;
pub use bdb_dump::*;
mod digest_utils;
pub use digest_utils::*;
mod exec;
pub use exec::*;
mod file_args;
pub use file_args::*;
mod parse_macro;
mod parser;
pub use parser::*;
mod parsing;
pub use parsing::*;
mod sapling;
mod string_utils;
pub use string_utils::*;
mod styles;
mod zcashd;
mod zewif;
mod zingo;
pub use zewif::*;

use clap::{Parser as ClapParser, Subcommand};

/// A tool for migrating Zcash wallets
#[derive(Debug, clap::Parser)]
#[command(author, version)]
#[command(propagate_version = true)]
#[command(styles=styles::get_styles())]
#[doc(hidden)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Debug, Subcommand)]
#[doc(hidden)]
enum MainCommands {
    Zcashd(zcashd::CommandArgs),
    Zingo(zingo::CommandArgs),
}

#[doc(hidden)]
fn main() {
    if let Err(e) = inner_main() {
        eprintln!("---");
        eprintln!("🔴 Error: {}\n", e);
        // Print the error context chain
        for cause in e.chain().skip(1) {
            eprintln!("Caused by: {}", cause);
        }
        std::process::exit(1);
    }
}

#[doc(hidden)]
fn inner_main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let output = match cli.command {
        MainCommands::Zcashd(args) => args.exec(),
        MainCommands::Zingo(args) => args.exec(),
    };
    let output = output?;
    if !output.is_empty() {
        println!("{}", output);
    }
    Ok(())
}
