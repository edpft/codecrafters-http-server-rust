use std::fmt;

use crate::error::Error;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Version {
    #[default]
    OnePointOne,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OnePointOne => write!(f, "HTTP/1.1"),
        }
    }
}

impl<'a> TryFrom<&'a str> for Version {
    type Error = Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "HTTP/1.1" => Ok(Self::OnePointOne),
            version if matches!(version, "HTTP/0.9" | "HTTP/1.0" | "HTTP/2" | "HTTP/3") => {
                Err(Error::UnsupportedVersion(version))
            }
            version => Err(Error::InvalidVersion(version)),
        }
    }
}
