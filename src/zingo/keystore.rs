use super::{UnifiedKeystore, Version1Keystore, Version2Keystore};

#[derive(Debug, Clone, Default)]
pub enum Keystore {
    #[default]
    None,
    Version1(Box<Version1Keystore>),
    Version2(Box<Version2Keystore>),
    Unified(Box<UnifiedKeystore>),
}
