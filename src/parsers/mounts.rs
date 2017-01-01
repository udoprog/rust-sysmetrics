use std::u8;
use std::str::{self, FromStr};
use nom::digit;

fn decode_path(input: &str) -> Result<String, ()> {
    let mut it = input.chars();
    let mut out = String::new();

    while let Some(c) = it.next() {
        match c {
            '\\' => {
                if let (Some(a), Some(b), Some(c)) = (it.next(), it.next(), it.next()) {
                    let mut codepoint: u32 = 0;
                    codepoint += a.to_digit(8).ok_or(())? << 6;
                    codepoint += b.to_digit(8).ok_or(())? << 3;
                    codepoint += c.to_digit(8).ok_or(())? << 0;

                    // overflow
                    if codepoint > u8::MAX as u32 {
                        return Err(());
                    }

                    out.push(codepoint as u8 as char);
                } else {
                    return Err(());
                }
            }
            c => {
                out.push(c);
            }
        }
    }

    Ok(out)
}

named!(pub type_u64<u64>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

#[derive(Debug, Default, PartialEq)]
pub struct Mount {
    /// Device
    pub device: String,
    /// Mountpoint
    pub mountpoint: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode_path() {
        assert_eq!(decode_path("foo\\134bar"), Ok("foo\\bar".to_owned()));
        assert_eq!(decode_path("foo\\040bar"), Ok("foo bar".to_owned()));
        assert_eq!(decode_path("foo\\011bar"), Ok("foo\tbar".to_owned()));
        assert_eq!(decode_path("foo\\012bar"), Ok("foo\nbar".to_owned()));
    }
}
