use anyhow::{bail, Result};

use crate::{parse, u256, Parse, Parser};

#[derive(Clone, Copy, PartialEq)]
pub enum Language {
    English = 0,
    SimplifiedChinese = 1,
    TraditionalChinese = 2,
    Czech = 3,
    French = 4,
    Italian = 5,
    Japanese = 6,
    Korean = 7,
    Portuguese = 8,
    Spanish = 9,
}

impl Language {
    pub fn from_u32(value: u32) -> Result<Self> {
        match value {
            0 => Ok(Language::English),
            1 => Ok(Language::SimplifiedChinese),
            2 => Ok(Language::TraditionalChinese),
            3 => Ok(Language::Czech),
            4 => Ok(Language::French),
            5 => Ok(Language::Italian),
            6 => Ok(Language::Japanese),
            7 => Ok(Language::Korean),
            8 => Ok(Language::Portuguese),
            9 => Ok(Language::Spanish),
            _ => bail!("Invalid language value: {}", value),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::SimplifiedChinese => "SimplifiedChinese",
            Language::TraditionalChinese => "TraditionalChinese",
            Language::Czech => "Czech",
            Language::French => "French",
            Language::Italian => "Italian",
            Language::Japanese => "Japanese",
            Language::Korean => "Korean",
            Language::Portuguese => "Portuguese",
            Language::Spanish => "Spanish",
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl std::fmt::Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Parse for Language {
    fn parse(p: &mut Parser) -> Result<Self> {
        let value = parse!(p, "value")?;
        Language::from_u32(value)
    }
}

#[derive(Clone, PartialEq)]
pub struct MnemonicSeed {
    pub language: Language,
    pub mnemonic: String,
    pub fingerprint: Option<u256>,
}

impl MnemonicSeed {
    pub fn set_fingerprint(mut self, fingerprint: u256) -> Self {
        self.fingerprint = Some(fingerprint);
        self
    }
}

impl std::fmt::Debug for MnemonicSeed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MnemonicSeed")
            .field("language", &self.language)
            .field("mnemonic", &self.mnemonic)
            .field("fingerprint", &&self.fingerprint)
            .finish()
    }
}

impl Parse for MnemonicSeed {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self {
            language: parse!(p, "language")?,
            mnemonic: parse!(p, "mnemonic")?,
            fingerprint: None,
        })
    }
}
