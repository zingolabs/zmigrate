use std::collections::HashMap;

use bc_components::Digest;
use bc_envelope::prelude::*;

#[derive(Debug, Clone)]
pub struct Attachments {
    envelopes: HashMap<Digest, Envelope>,
}

impl Default for Attachments {
    fn default() -> Self {
        Self::new()
    }
}

impl Attachments {
    pub fn new() -> Self {
        Self {
            envelopes: HashMap::new(),
        }
    }

    pub fn add(&mut self, attachment: Envelope) {
        self.envelopes.insert(attachment.digest().into_owned(), attachment);
    }

    pub fn get(&self, digest: &Digest) -> Option<&Envelope> {
        self.envelopes.get(digest)
    }

    pub fn remove(&mut self, digest: &Digest) -> Option<Envelope> {
        self.envelopes.remove(digest)
    }

    pub fn clear(&mut self) {
        self.envelopes.clear();
    }
}
