use std::fmt::Display;
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    Xci,
    Nsp,
}

impl FileType {
    pub fn glob_pattern(self) -> &'static str {
        match self {
            Self::Xci => "xc[0-9]",
            Self::Nsp => "ns[0-9]",
        }
    }

    pub fn extension(self) -> &'static str {
        match self {
            Self::Xci => "xci",
            Self::Nsp => "nsp",
        }
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nsp => write!(f, "nsp"),
            Self::Xci => write!(f, "xci"),
        }
    }
}

impl FromStr for FileType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xci" => Ok(Self::Xci),
            "nsp" => Ok(Self::Nsp),
            i if i.starts_with("xc") => Ok(Self::Xci),
            i if i.starts_with("ns") => Ok(Self::Nsp),
            _ => Err(Error::InvalidExtension(s.to_string())),
        }
    }
}
