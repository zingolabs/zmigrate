use super::{u256, MnemonicLanguage};

pub struct Bip39Mnemonic {
    mnemonic: String,
    language: Option<MnemonicLanguage>,
    fingerprint: Option<u256>,
}

impl std::fmt::Debug for Bip39Mnemonic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("MnemonicSeed")
            .field("language", &self.language)
            .field("mnemonic", &self.mnemonic)
            .field("fingerprint", &&self.fingerprint)
            .finish()
    }
}

impl Bip39Mnemonic {
    pub fn new(mnemonic: String, language: Option<MnemonicLanguage>, fingerprint: Option<u256>) -> Self {
        Self { mnemonic, language, fingerprint }
    }

    pub fn set_fingerprint(mut self, fingerprint: u256) -> Self {
        self.fingerprint = Some(fingerprint);
        self
    }

    pub fn mnemonic(&self) -> &String {
        &self.mnemonic
    }

    pub fn set_mnemonic(&mut self, mnemonic: String) {
        self.mnemonic = mnemonic;
    }

    pub fn language(&self) -> Option<&MnemonicLanguage> {
        self.language.as_ref()
    }

    pub fn fingerprint(&self) -> Option<&u256> {
        self.fingerprint.as_ref()
    }

    pub fn set_language(&mut self, language: MnemonicLanguage) {
        self.language = Some(language);
    }
}
