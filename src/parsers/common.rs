use nom::*;
use std::str::{self, FromStr};

named!(pub type_u64<u64>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

#[cfg(test)]
mod test {
    use super::*;
}
