use std::fmt;

use nom::{branch, bytes, combinator, sequence, IResult};

// use crate::error::Error;

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Evolution_of_HTTP
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Version {
    ZeroPointNine,
    OnePointZero,
    #[default]
    OnePointOne,
    Two,
    Three,
}

impl Version {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        sequence::preceded(http, sem_ver)(bytes)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ZeroPointNine => write!(f, "HTTP/0.9"),
            Self::OnePointZero => write!(f, "HTTP/1.0"),
            Self::OnePointOne => write!(f, "HTTP/1.1"),
            Self::Two => write!(f, "HTTP/2"),
            Self::Three => write!(f, "HTTP/3"),
        }
    }
}

fn http(bytes: &[u8]) -> IResult<&[u8], &[u8]> {
    bytes::complete::tag(b"HTTP/")(bytes)
}

fn sem_ver(bytes: &[u8]) -> IResult<&[u8], Version> {
    branch::alt((
        combinator::map(bytes::complete::tag(b"0.9"), |_| Version::ZeroPointNine),
        combinator::map(bytes::complete::tag(b"1.0"), |_| Version::OnePointZero),
        combinator::map(bytes::complete::tag(b"1.1"), |_| Version::OnePointOne),
        combinator::map(bytes::complete::tag(b"2"), |_| Version::Two),
        combinator::map(bytes::complete::tag(b"3"), |_| Version::Three),
    ))(bytes)
}
