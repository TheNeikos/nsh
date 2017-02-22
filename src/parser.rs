use nom::{is_space, multispace};

use types::Command;

named!(string_between_quotes<Command>, alt!(
    map!(
        delimited!(char!('\''), is_not!("\'"), char!('\'')),
        Command::to_argument
    ) | 
    map!(
        delimited!(char!('\"'), is_not!("\""), char!('\"')),
        Command::to_parsed_argument
    )
));

named!(pub command<(Command, Vec<Command>)>, tuple!(
    map!(
        take_till!(is_space),
        Command::to_executable
    ),
    do_parse!(
        opt!(take_while!(is_space)) >>
        args: arguments             >>
        opt!(take_while!(is_space)) >>
        (args)
    )
));

named!(pipe<Command>, map!(
    do_parse!(
        opt!(take_while!(is_space))                                     >>
        char!('|')                                                      >>
        opt!(take_while!(is_space))                                     >>
        rest: take_till!(alt!( eof!() | char!('|')))                     >>
        args: dbg_dmp!(terminated!(command, opt!(char!('|'))))   >>
        (args)
    ),
    Command::to_pipe
));

named!(redirection<Command>, do_parse!(
        opt!(take_while!(is_space)) >>
        char!('>')                   >>
        opt!(take_while!(is_space)) >>
        arg: simple_argument        >>
        (arg)
));

named!(string<Command>, map!(take_till!(is_space), Command::to_argument));

named!(simple_argument<Command>, alt!(
    string_between_quotes | string
));

named!(pub argument<Command>,
    preceded!(
        opt!(multispace),
        alt!( pipe | redirection | simple_argument )
    )
);

named!(pub arguments< Vec<Command> >, 
    many0!( 
        argument
    )
);


#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::OsString;
    use nom::IResult;

    use types::Command;

    macro_rules! oss {
        ($n:expr) => {{
            let mut a = OsString::new();
            a.push($n);
            a
        }}
    }

    macro_rules! com {
        (n $n:expr) => {Command::Argument(oss!($n))};
        (p $n:expr) => {Command::ParsedArgument(oss!($n))};
        (e $n:expr) => {Command::Executable(oss!($n))};
        (r $n:expr) => {Command::Redirection(oss!($n))};
        (| ($e:expr, $n:expr)) => {Command::Pipe(Box::new(($e, $n)))};
    }
    
    #[test]
    fn parse_command() {
        let ret = command(b"ls -l");
        assert_eq!(ret, IResult::Done(&b""[..], (com!(e "ls"), vec![com!(n "-l")])));
    }

    #[test]
    fn parse_command_complicated() {
        let ret = command(b"grep -P '.?|(..+?)\\1+' /dev/urandom");
        assert_eq!(ret, IResult::Done(&b""[..], 
            (com!(e "grep"), vec![
                com!(n "-P"), com!(n ".?|(..+?)\\1+"), com!(n "/dev/urandom")
            ])));
    }

    #[test]
    fn parse_command_pipes() {
        let ret = command(b"ip addr | grep inet | awk '{ print $2 }' | sort");
        assert_eq!(ret, IResult::Done(&b""[..], 
            (com!(e "ip"), vec![com!(n "addr"), 
                com!(| (com!(e "grep"), vec![com!(n "inet")])),
                com!(| (com!(e "awk"),  vec![com!(n "{ print $2 }")])),
                com!(| (com!(e "sort"), vec![]))
            ])));
    }

    #[test]
    fn parse_argument() {
        let ret = argument(b"-l");
        assert_eq!(ret, IResult::Done(&b""[..], com!(n "-l")));
    }

    #[test]
    fn parse_arguments_empty() {
        let ret = arguments(b"");
        assert_eq!(ret, IResult::Done(&b""[..], Vec::new()));
    }

    #[test]
    fn parse_arguments() {
        let ret = arguments(b"-l -h");
        assert_eq!(ret, IResult::Done(&b""[..], vec![com!(n "-l"), com!(n "-h")]));
    }

    #[test]
    fn parse_arguments_with_quotes() {
        let ret = arguments(b"-l \"-h -g\"");
        assert_eq!(ret, IResult::Done(&b""[..], vec![com!(n "-l"), com!(p "-h -g")]));

        let ret = arguments(b"-l '-h -g'");
        assert_eq!(ret, IResult::Done(&b""[..], vec![com!(n "-l"), com!(n "-h -g")]));
    }
}
