use std::fmt;

use nom::{branch, bytes, combinator, IResult};

// https://www.rfc-editor.org/rfc/rfc9110.html#table-4
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Method {
    #[default]
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
}

impl Method {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        branch::alt((
            combinator::map(bytes::complete::tag(b"GET"), |_| Self::Get),
            combinator::map(bytes::complete::tag(b"HEAD"), |_| Self::Head),
            combinator::map(bytes::complete::tag(b"POST"), |_| Self::Post),
            combinator::map(bytes::complete::tag(b"PUT"), |_| Self::Put),
            combinator::map(bytes::complete::tag(b"DELETE"), |_| Self::Delete),
            combinator::map(bytes::complete::tag(b"CONNECT"), |_| Self::Connect),
            combinator::map(bytes::complete::tag(b"OPTIONS"), |_| Self::Options),
            combinator::map(bytes::complete::tag(b"TRACE"), |_| Self::Trace),
        ))(bytes)
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Connect => "CONNECT",
            Method::Options => "OPTIONS",
            Method::Trace => "TRACE",
        };
        write!(f, "{text}")
    }
}
