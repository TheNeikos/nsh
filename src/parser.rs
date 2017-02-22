use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use nom::{is_space, multispace};

use types::Arg;

named!(string_between_quotes<Arg>, alt!(
    map!(
        delimited!(char!('\''), is_not!("\'"), char!('\'')),
        Arg::to_normal
    ) | 
    map!(
        delimited!(char!('\"'), is_not!("\""), char!('\"')),
        Arg::to_processed
    )
));
named!(cont_string, take_till!(is_space));

named!(pub command<(&OsStr, Vec<Arg>)>, tuple!(
    map!(
        take_till!(is_space),
        OsStr::from_bytes
    ),
    do_parse!(
        opt!(take_while!(is_space)) >>
        args: arguments             >>
        opt!(take_while!(is_space)) >>
        (args)
    )
));

named!(argument_w_or_wo_q<Arg>, alt!( string_between_quotes | map!(cont_string, Arg::to_normal) ) );

named!(pub argument<Arg>,
    preceded!(
        opt!(multispace),
        argument_w_or_wo_q
    )
);

named!(pub arguments< Vec<Arg> >, 
    many0!( 
        argument
    )
);


#[cfg(test)]
mod test {
    use super::*;
    use std::ffi::OsString;
    use nom::IResult;

    use types::Arg;

    macro_rules! oss {
        ($n:expr) => {{
            let mut a = OsString::new();
            a.push($n);
            a
        }}
    }

    macro_rules! arg {
        (n $n:expr) => {Arg::Normal(oss!($n))};
        (p $n:expr) => {Arg::Processed(oss!($n))};
    }
    
    #[test]
    fn parse_command() {
        let ret = command(b"ls -l");
        assert_eq!(ret, IResult::Done(&b""[..], (&oss!("ls")[..], vec![arg!(n "-l")])));
    }

    #[test]
    fn parse_command_complicated() {
        let ret = command(b"grep -P '.?|(..+?)\\1+' /dev/urandom");
        assert_eq!(ret, IResult::Done(&b""[..], 
            (&oss!("grep")[..], vec![
                arg!(n "-P"), arg!(n ".?|(..+?)\\1+"), arg!(n "/dev/urandom")
            ])));
    }

    #[test]
    fn parse_argument() {
        let ret = argument(b"-l");
        assert_eq!(ret, IResult::Done(&b""[..], arg!(n "-l")));
    }

    #[test]
    fn parse_arguments_empty() {
        let ret = arguments(b"");
        assert_eq!(ret, IResult::Done(&b""[..], Vec::new()));
    }

    #[test]
    fn parse_arguments() {
        let ret = arguments(b"-l -h");
        assert_eq!(ret, IResult::Done(&b""[..], vec![arg!(n "-l"), arg!(n "-h")]));
    }

    #[test]
    fn parse_arguments_with_quotes() {
        let ret = arguments(b"-l \"-h -g\"");
        assert_eq!(ret, IResult::Done(&b""[..], vec![arg!(n "-l"), arg!(p "-h -g")]));

        let ret = arguments(b"-l '-h -g'");
        assert_eq!(ret, IResult::Done(&b""[..], vec![arg!(n "-l"), arg!(n "-h -g")]));
    }
}
