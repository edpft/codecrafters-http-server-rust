use std::fmt;

#[derive(Debug)]
pub enum RequestError<'a> {
    Parsing(&'a [u8]),
    NotEnoughRequestParts(u8),
    InvalidMethod(&'a str),
    InvalidTarget(&'a str),
    InvalidVersion(&'a str),
    UnsupportedVersion(&'a str),
    TooManyRequestLineParts(u8),
    NotEnoughRequestLineParts(u8),
    InvalidHeaderNameValuePair(&'a str),
    InvalidHeaderName(&'a str),
}

impl<'a> fmt::Display for RequestError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parsing(bytes) => write!(f, "Unable to parse &[u8] into &str: {bytes:?}"),
            Self::NotEnoughRequestParts(number_of_parts) => write!(
                f,
                "Expecting request to have at least 2 parts (request line, zero or more headers ), but received {number_of_parts}"
            ),
            Self::InvalidMethod(method) => write!(f, "{method} is not a valid HTTP method"),
            Self::InvalidTarget(target) => write!(f, "{target} is not a valid target"),
            Self::InvalidVersion(version) => write!(f, "{version} is not a valid HTTP version"),
            Self::UnsupportedVersion(version) => write!(f, "{version} is not supported"),
            Self::TooManyRequestLineParts(number_of_parts)
            | Self::NotEnoughRequestLineParts(number_of_parts) => write!(
                f,
                "Expecting request line to have 3 parts (HTTP method, request target, and HTTP version), but received {number_of_parts}"
            ),
            Self::InvalidHeaderNameValuePair(key_value_pair) => write!(f, "{key_value_pair} is not a valid header name - header value pair because it does not contain a ':' "),
            Self::InvalidHeaderName(header_name) => write!(f, "{header_name} is not a valid header name")
        }
    }
}
