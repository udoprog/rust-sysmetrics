use nom::*;
use std::str::{self, FromStr};

named!(pub type_u64<u64>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(pub type_proc_path<&str>,
       map_res!(take_until!(" "), str::from_utf8));

named!(mnt_escape<char>, do_parse!(
    numbers: preceded!(tag!("\\"), map_res!(take!(3), str::from_utf8)) >>
    (
        u8::from_str_radix(numbers, 8).unwrap() as char
    )
));

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_one() {
        assert_eq!(mnt_escape(b"\\134"), IResult::Done(&b""[..], '\\'));
        assert_eq!(mnt_escape(b"\\040"), IResult::Done(&b""[..], ' '));
        assert_eq!(mnt_escape(b"\\011"), IResult::Done(&b""[..], '\t'));
        assert_eq!(mnt_escape(b"\\012"), IResult::Done(&b""[..], '\n'));
    }
}
