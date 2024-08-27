use std::fmt;

use crate::error::RequestError;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum HttpVersion {
    #[default]
    OnePointOne,
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OnePointOne => write!(f, "HTTP/1.1"),
        }
    }
}

impl<'a> TryFrom<&'a str> for HttpVersion {
    type Error = RequestError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "HTTP/1.1" => Ok(Self::OnePointOne),
            version if matches!(version, "HTTP/0.9" | "HTTP/1.0" | "HTTP/2" | "HTTP/3") => {
                Err(RequestError::UnsupportedVersion(version))
            }
            version => Err(RequestError::InvalidVersion(version)),
        }
    }
}
