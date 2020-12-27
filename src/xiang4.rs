use std::convert::From;
use std::fmt;

use actix_web::http::StatusCode;

use super::errors::{Error, Result};

pub enum Xiang4 {
    TaiYang,
    ShaoYin,
    ShaoYang,
    TaiYin,
}

impl fmt::Debug for Xiang4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TaiYang => "太陽",
                Self::ShaoYin => "少陰",
                Self::ShaoYang => "少陽",
                Self::TaiYin => "太陰",
            }
        )
    }
}

impl fmt::Display for Xiang4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TaiYang => "\u{268C}",
                Self::ShaoYin => "\u{268D}",
                Self::ShaoYang => "\u{268E}",
                Self::TaiYin => "\u{268F}",
            }
        )
    }
}

impl Xiang4 {
    pub fn new(v: u16) -> Result<Self> {
        match v {
            0x268C => Ok(Self::TaiYang),
            0x268D => Ok(Self::ShaoYin),
            0x268E => Ok(Self::ShaoYang),
            0x268F => Ok(Self::TaiYin),
            _ => Err(Error::Http(
                StatusCode::BAD_REQUEST,
                Some(format!("bad xiang4 number {}", v)),
            )),
        }
    }
}

impl From<Xiang4> for u16 {
    fn from(item: Xiang4) -> Self {
        match item {
            Xiang4::TaiYang => 0x268C,
            Xiang4::ShaoYin => 0x268D,
            Xiang4::ShaoYang => 0x268E,
            Xiang4::TaiYin => 0x268F,
        }
    }
}
