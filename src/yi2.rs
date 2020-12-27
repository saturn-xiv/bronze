use std::convert::From;
use std::fmt;

use actix_web::http::StatusCode;

use super::errors::{Error, Result};

pub enum Yi2 {
    Yang,
    Yin,
}

impl fmt::Debug for Yi2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yang => "陽",
                Self::Yin => "陰",
            }
        )
    }
}

impl fmt::Display for Yi2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Yang => "\u{268A}",
                Self::Yin => "\u{268B}",
            }
        )
    }
}

impl Yi2 {
    pub fn new(v: u16) -> Result<Self> {
        match v {
            0x268A => Ok(Self::Yang),
            0x268B => Ok(Self::Yin),
            _ => Err(Error::Http(
                StatusCode::BAD_REQUEST,
                Some(format!("bad yi2 number {}", v)),
            )),
        }
    }
}

impl From<Yi2> for u16 {
    fn from(item: Yi2) -> Self {
        match item {
            Yi2::Yang => 0x268A,
            Yi2::Yin => 0x268B,
        }
    }
}
