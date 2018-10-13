use super::token::SliceRange;
use std::convert::From;
use std::ops::Deref;
use std::{fmt, str};

// NOTE: a thin wrapper around token's raw bytes subslice that allows us pretty print tokens
pub struct RawSubslice<'t>(&'t [u8]);

impl<'t> RawSubslice<'t> {
    pub fn as_str(&self) -> &str {
        str::from_utf8(self).unwrap()
    }

    pub fn as_string(&self) -> String {
        String::from_utf8(self.to_vec()).unwrap()
    }
}

impl<'t> From<&'t [u8]> for RawSubslice<'t> {
    fn from(bytes: &'t [u8]) -> Self {
        RawSubslice(bytes)
    }
}

impl<'t> From<(&'t [u8], SliceRange)> for RawSubslice<'t> {
    fn from((raw, range): (&'t [u8], SliceRange)) -> Self {
        (&raw[range.start..range.end]).into()
    }
}

impl<'t> fmt::Debug for RawSubslice<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "`{}`", self.as_str())
    }
}

impl<'t> Deref for RawSubslice<'t> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.0
    }
}
