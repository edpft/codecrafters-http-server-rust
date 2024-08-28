use std::{collections::HashMap, fmt, str};

use nom::{branch, bytes::complete, combinator, multi, sequence::Tuple, IResult};

use crate::parsing_utils;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Headers(HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub fn new(headers: HashMap<HeaderName, HeaderValue>) -> Self {
        Self(headers)
    }

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let header_sequence = multi::many0(Header::parse);
        let (remainder, (header_sequence, _)) =
            (header_sequence, parsing_utils::crlf).parse(bytes)?;
        let headers: Headers = header_sequence.into_iter().collect();
        Ok((remainder, headers))
    }

    pub fn set_accept(mut self, accept: impl Into<HeaderValue>) -> Self {
        let accept = accept.into();
        self.insert(HeaderName::Accept, accept);
        self
    }

    pub fn set_content_length(mut self, content_length: usize) -> Self {
        let content_length_string = content_length.to_string();
        let content_length = HeaderValue::new(content_length_string);
        self.insert(HeaderName::ContentLength, content_length);
        self
    }

    pub fn set_content_type(mut self, content_type: impl Into<HeaderValue>) -> Self {
        let content_type = content_type.into();
        self.insert(HeaderName::ContentType, content_type);
        self
    }

    pub fn set_host(mut self, host: impl Into<HeaderValue>) -> Self {
        let host = host.into();
        self.insert(HeaderName::Host, host);
        self
    }

    pub fn set_user_agent(mut self, user_agent: impl Into<HeaderValue>) -> Self {
        let user_agent = user_agent.into();
        self.insert(HeaderName::UserAgent, user_agent);
        self
    }

    pub fn user_agent(&self) -> Option<&HeaderValue> {
        self.0.get(&HeaderName::UserAgent)
    }

    fn insert(&mut self, header_name: HeaderName, header_value: HeaderValue) {
        self.0.insert(header_name, header_value);
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .iter()
            .fold(f, |formatter, (header_name, header_value)| {
                let _ = write!(formatter, "{header_name}: {header_value}\r\n");
                formatter
            });
        Ok(())
    }
}

impl FromIterator<Header> for Headers {
    fn from_iter<T: IntoIterator<Item = Header>>(iter: T) -> Self {
        let mut headers = Self::default();

        for Header((header_name, header_value)) in iter {
            headers.insert(header_name, header_value);
        }

        headers
    }
}

// `HeaderName` implements `Copy` but `HeaderValue` and `Headers` do not
#[allow(clippy::copy_iterator)]
impl<'a> Iterator for &'a Headers {
    type Item = (&'a HeaderName, &'a HeaderValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Header(pub (HeaderName, HeaderValue));

impl Header {
    pub fn new(header_name: HeaderName, header_value: HeaderValue) -> Self {
        Self((header_name, header_value))
    }

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remainder, (header_name, _, _, header_value, _)) = (
            HeaderName::parse,
            parsing_utils::colon,
            parsing_utils::space,
            HeaderValue::parse,
            parsing_utils::crlf,
        )
            .parse(bytes)?;
        let header = Header::new(header_name, header_value);
        Ok((remainder, header))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HeaderName {
    Host,
    UserAgent,
    Accept,
    ContentType,
    ContentLength,
}

impl HeaderName {
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        branch::alt((
            combinator::map(complete::tag(b"Host"), |_| Self::Host),
            combinator::map(complete::tag(b"User-Agent"), |_| Self::UserAgent),
            combinator::map(complete::tag(b"Accept"), |_| Self::Accept),
            combinator::map(complete::tag(b"Content-Type"), |_| Self::ContentType),
            combinator::map(complete::tag(b"Content-Length"), |_| Self::ContentLength),
        ))(bytes)
    }
}

impl fmt::Display for HeaderName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Self::Host => "Host",
            Self::UserAgent => "User-Agent",
            Self::Accept => "Accept",
            Self::ContentType => "Content-Type",
            Self::ContentLength => "Content-Length",
        };
        write!(f, "{text}")
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HeaderValue(String);

impl HeaderValue {
    pub fn new(header_value: impl Into<String>) -> Self {
        let header_value = header_value.into();
        Self(header_value)
    }

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remainder, header_value_bytes) = complete::take_until1("\r\n")(bytes)?;
        let header_value_string =
            str::from_utf8(header_value_bytes).expect("HeaderValue string is valid UTF8");
        let header_value = Self::new(header_value_string);
        Ok((remainder, header_value))
    }
}

impl fmt::Display for HeaderValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ContentType> for HeaderValue {
    fn from(content_type: ContentType) -> Self {
        match content_type {
            ContentType::Text => HeaderValue::new("text/plain"),
        }
    }
}

impl From<&str> for HeaderValue {
    fn from(header_value: &str) -> Self {
        HeaderValue::new(header_value)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum ContentType {
    #[default]
    Text,
}

#[cfg(test)]
mod tests {
    use core::str;

    use super::*;

    #[test]
    fn deserialise_empty_headers() {
        let bytes = b"\r\n";
        let expected_headers = Headers::default();

        let (remainder, headers) = Headers::parse(bytes).unwrap_or_else(|_| {
            panic!(
                "{} is a valid sequence of headers",
                str::from_utf8(bytes).unwrap()
            )
        });

        assert!(remainder.is_empty());
        assert_eq!(headers, expected_headers);
    }

    #[test]
    fn deserialise_host_header() {
        let bytes = b"Host: localhost:4221\r\n";
        let expected_header = Header::new(HeaderName::Host, HeaderValue::new("localhost:4221"));

        let (remainder, header) = Header::parse(bytes)
            .unwrap_or_else(|_| panic!("{} is a valid header", str::from_utf8(bytes).unwrap()));

        assert!(remainder.is_empty());
        assert_eq!(header, expected_header);
    }

    #[test]
    fn deserialise_valid_request_headers() {
        let bytes = b"Host: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";
        let expected_headers = Headers::default()
            .set_host("localhost:4221")
            .set_user_agent("curl/7.64.1")
            .set_accept("*/*");

        let (remainder, headers) = Headers::parse(bytes).unwrap_or_else(|_| {
            panic!(
                "{} is a valid sequence of headers",
                str::from_utf8(bytes).unwrap()
            )
        });

        assert!(remainder.is_empty());
        assert_eq!(headers, expected_headers);
    }

    #[test]
    fn serialize_deserialize_valid_request_headers() {
        let headers = Headers::default()
            .set_host("localhost:4221")
            .set_user_agent("curl/7.64.1")
            .set_accept("*/*");

        let serialized_headers = headers.to_string();

        let (remainder, deserialized_headers) =
            Headers::parse(serialized_headers.as_bytes()).expect("Serialized headers are valid");

        assert!(remainder.is_empty());
        assert_eq!(headers, deserialized_headers);
    }

    #[test]
    fn serialize_deserialize_valid_response_headers() {
        let headers = Headers::default()
            .set_content_type(ContentType::Text)
            .set_content_length(3);

        let serialized_headers = headers.to_string();

        let (remainder, deserialized_headers) =
            Headers::parse(serialized_headers.as_bytes()).expect("Serialized headers are valid");

        assert!(remainder.is_empty());
        assert_eq!(headers, deserialized_headers);
    }
}
