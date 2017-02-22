use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug, Eq, PartialEq)]
pub enum Arg {
    Normal(OsString),
    Processed(OsString)
}

impl Arg {
    pub fn to_normal(b: &[u8]) -> Arg {
        let mut s = OsString::new();
        s.push(OsStr::from_bytes(b));
        Arg::Normal(s)
    }

    pub fn to_processed(b: &[u8]) -> Arg {
        let mut s = OsString::new();
        s.push(OsStr::from_bytes(b));
        Arg::Processed(s)
    }
}
