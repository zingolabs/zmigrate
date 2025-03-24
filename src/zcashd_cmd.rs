use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Args;
use std::fmt::Write;

use crate::file_args::{FileArgs, FileArgsLike};

use zewif_zcashd::{BDBDump, ZcashdDump, ZcashdParser};

/// Doc comment here
#[derive(Debug, Args)]
#[group(skip)]
pub struct CommandArgs {
    #[command(flatten)]
    file_args: FileArgs,
}

impl FileArgsLike for CommandArgs {
    fn file(&self) -> &PathBuf {
        &self.file_args.file
    }
}

impl crate::exec::Exec for CommandArgs {
    fn exec(&self) -> Result<String> {
        let db_dump = BDBDump::from_file(self.file()).context("Parsing BerkeleyDB file")?;

        let zcashd_dump = ZcashdDump::from_bdb_dump(&db_dump).context("Parsing Zcashd dump")?;

        let (zcashd_wallet, unparsed_keys) =
            ZcashdParser::parse_dump(&zcashd_dump).context("Parsing Zcashd dump")?;

        let mut output = String::new();

        writeln!(output, "{}", zcashd_dump.keyname_summary())?;

        // writeln!(output, "---")?;
        // writeln!(output, "{}", zcashd_dump.dump_keys())?;

        writeln!(output, "---")?;
        writeln!(output, "{:#?}", zcashd_wallet)?;

        if unparsed_keys.is_empty() {
            writeln!(output, "---")?;
            writeln!(output, "✅ All keys parsed successfully")
                .context("Writing to output buffer")?;
        } else {
            writeln!(output, "---")?;
            writeln!(output, "🛑 Unparsed keys:")?;
            let mut sorted_keys: Vec<_> = unparsed_keys.into_iter().collect();
            sorted_keys.sort();
            let mut last_keyname: Option<String> = None;
            for key in sorted_keys {
                if let Some(ref last_keyname) = last_keyname {
                    if *last_keyname != key.keyname {
                        writeln!(output)?;
                    }
                }
                last_keyname = Some(key.keyname.to_string());

                let value = zcashd_dump.value_for_key(&key)?;
                writeln!(output, "❌ key: {}\n\tvalue: {}", key, value)?;
            }
        }

        Ok(output)
    }
}
