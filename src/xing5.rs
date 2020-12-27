use std::fmt;

pub enum Xing5 {
    Jin,
    Mu,
    Shui,
    Huo,
    Tu,
}

impl fmt::Debug for Xing5 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Jin => "金",
                Self::Mu => "木",
                Self::Shui => "水",
                Self::Huo => "火",
                Self::Tu => "土",
            }
        )
    }
}

impl fmt::Display for Xing5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Jin => "金",
                Self::Mu => "木",
                Self::Shui => "水",
                Self::Huo => "火",
                Self::Tu => "土",
            }
        )
    }
}
