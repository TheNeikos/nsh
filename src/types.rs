use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    Executable(OsString),
    Argument(OsString),
    ParsedArgument(OsString),
    Pipe(Box<(Command, Vec<Command>)>),
    Redirection(OsString),
}

impl Command {

    pub fn to_pipe(a: (Command, Vec<Command>)) -> Command {
        Command::Pipe(Box::new(a))
    }

    pub fn to_executable(b: &[u8]) -> Command {
        let mut s = OsString::new();
        s.push(OsStr::from_bytes(b));
        Command::Executable(s)
    }

    pub fn to_redirection(b: &[u8]) -> Command {
        let mut s = OsString::new();
        s.push(OsStr::from_bytes(b));
        Command::Redirection(s)
    }

    pub fn to_argument(b: &[u8]) -> Command {
        let mut s = OsString::new();
        s.push(OsStr::from_bytes(b));
        Command::Argument(s)
    }

    pub fn to_parsed_argument(b: &[u8]) -> Command {
        let mut s = OsString::new();
        s.push(OsStr::from_bytes(b));
        Command::ParsedArgument(s)
    }
}
