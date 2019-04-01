use crate::base::{Bytes, Chunk, Range};
use crate::html::LocalNameHash;
use std::fmt::{self, Debug};

#[derive(Copy, Clone)]
pub struct TagNameInfo<'i> {
    input: &'i Chunk<'i>,
    name_range: Range,
    name_hash: LocalNameHash,
}

impl<'i> TagNameInfo<'i> {
    #[inline]
    pub fn new(input: &'i Chunk<'i>, name_range: Range, name_hash: LocalNameHash) -> Self {
        TagNameInfo {
            input,
            name_range,
            name_hash,
        }
    }

    #[inline]
    pub fn name(&self) -> Bytes<'i> {
        self.input.slice(self.name_range)
    }

    #[inline]
    pub fn name_hash(&self) -> LocalNameHash {
        self.name_hash
    }
}

impl Debug for TagNameInfo<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TagNameInfo")
            .field("name", &self.name())
            .field("name_hash", &self.name_hash)
            .finish()
    }
}