use anyhow::Result;
use clap::Args;
use std::{io::Read, path::PathBuf};

use crate::Data;

pub trait FileArgsLike {
    fn file(&self) -> &PathBuf;

    fn read_file(&self) -> Result<Data> {
        let mut vec = Vec::new();
        std::fs::File::open(self.file())?.read_to_end(&mut vec)?;
        Ok(Data(vec))
    }
}

#[derive(Debug, Args)]
#[group(skip)]
pub struct FileArgs {
    pub file: PathBuf,
}
