use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::file_args::{FileArgs, FileArgsLike};

use crate::zingo::ZingoParser;

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
        let file_path = self.file();
        let file_data = std::fs::read(file_path)?.into();
        let mut parser = ZingoParser::new(&file_data);
        let wallet = parser.parse()?;
        Ok(format!("{:#?}", wallet))
    }
}
