use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use crate::file_args::{FileArgs, FileArgsLike};

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
        Ok("Hello, world!".to_string())
    }
}
