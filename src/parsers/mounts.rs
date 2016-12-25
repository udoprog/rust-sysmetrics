use std::str::{self, FromStr};
use nom::{line_ending, space, digit, alphanumeric, rest};

use ::parsers::common::*;

named!(pub type_u64<u64>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

#[derive(Debug, Default, PartialEq)]
pub struct Mount {
    /// Device
    pub device: String,
    /// Mountpoint
    pub mountpoint: String,
}
