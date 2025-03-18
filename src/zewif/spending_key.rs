use crate::Blob;

#[derive(Debug, Clone)]
pub struct SpendingKey(pub Blob<32>);
