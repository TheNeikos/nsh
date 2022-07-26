use std::ffi::{OsStr, OsString};
use std::collections::HashMap;
use std::os::unix::io::RawFd;

pub enum RawArgument {
    Literal(OsString),
    Variable(OsString),
    Environment(OsString)
}

pub enum Argument {
    Literal(OsString),
    Calculated(Vec<RawArgument>)
}

pub struct Command {
    executable: OsString,
    arguments: Vec<Argument>,
    redirections: HashMap<RawFd, Command>
}
