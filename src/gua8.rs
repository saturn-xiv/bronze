use std::convert::From;
use std::fmt;

use actix_web::http::StatusCode;

use super::errors::{Error, Result};

pub enum Gua8 {
    Qian,
    Dui,
    Li,
    Zhen,
    Xun,
    Kan,
    Gen,
    Kun,
}

impl fmt::Display for Gua8 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Qian => "\u{2630}",
                Self::Dui => "\u{2631}",
                Self::Li => "\u{2632}",
                Self::Zhen => "\u{2633}",
                Self::Xun => "\u{2634}",
                Self::Kan => "\u{2635}",
                Self::Gen => "\u{2636}",
                Self::Kun => "\u{2637}",
            }
        )
    }
}

impl fmt::Debug for Gua8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Qian => "乾三連",
                Self::Dui => "兌上缺",
                Self::Li => "離中虛",
                Self::Zhen => "震仰盂",
                Self::Xun => "巽下斷",
                Self::Kan => "坎中滿",
                Self::Gen => "艮覆碗",
                Self::Kun => "坤六斷",
            }
        )
    }
}

impl Gua8 {
    pub fn new(v: u16) -> Result<Self> {
        match v {
            0x2630 => Ok(Self::Qian),
            0x2631 => Ok(Self::Dui),
            0x2632 => Ok(Self::Li),
            0x2633 => Ok(Self::Zhen),
            0x2634 => Ok(Self::Xun),
            0x2635 => Ok(Self::Kan),
            0x2636 => Ok(Self::Gen),
            0x2637 => Ok(Self::Kun),
            _ => Err(Error::Http(
                StatusCode::BAD_REQUEST,
                Some(format!("bad gua8 number {}", v)),
            )),
        }
    }
}

impl From<Gua8> for u16 {
    fn from(item: Gua8) -> Self {
        match item {
            Gua8::Qian => 0x2630,
            Gua8::Dui => 0x2631,
            Gua8::Li => 0x2632,
            Gua8::Zhen => 0x2633,
            Gua8::Xun => 0x2634,
            Gua8::Kan => 0x2635,
            Gua8::Gen => 0x2636,
            Gua8::Kun => 0x2637,
        }
    }
}
