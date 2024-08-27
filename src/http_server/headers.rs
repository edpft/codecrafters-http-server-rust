use std::{collections::HashMap, fmt};

use crate::error::RequestError;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Headers(HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub fn new(headers: HashMap<HeaderName, HeaderValue>) -> Self {
        Self(headers)
    }
}

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\r\n")
    }
}

impl<'a> TryFrom<&'a str> for Headers {
    type Error = RequestError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut headers: HashMap<HeaderName, HeaderValue> = HashMap::default();
        for header_name_value_pair in value.split("\r\n") {
            let Some(mid) = header_name_value_pair.find(':') else {
                let error = RequestError::InvalidHeaderNameValuePair(header_name_value_pair);
                return Err(error);
            };
            let (key, value) = header_name_value_pair.split_at(mid);
            let header_name = HeaderName::try_from(key)?;
            let trimmed_header_value = value
                .strip_prefix(':')
                .expect("value must contained a ':'")
                .trim_start();
            let header_value = HeaderValue::new(trimmed_header_value);
            headers.insert(header_name, header_value);
        }
        let headers = Headers::new(headers);
        Ok(headers)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum HeaderName {
    Host,
    UserAgent,
    Accept,
}

impl<'a> TryFrom<&'a str> for HeaderName {
    type Error = RequestError<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "Host" => Ok(Self::Host),
            "User-Agent" => Ok(Self::UserAgent),
            "Accept" => Ok(Self::Accept),
            other => Err(RequestError::InvalidHeaderName(other)),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct HeaderValue(String);

impl HeaderValue {
    pub fn new(header_value: impl Into<String>) -> Self {
        let header_value = header_value.into();
        Self(header_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_headers() {
        let expected_headers = {
            let mut expected_headers: HashMap<HeaderName, HeaderValue> = HashMap::default();
            expected_headers.insert(HeaderName::Host, HeaderValue::new("localhost:4221"));
            expected_headers.insert(HeaderName::UserAgent, HeaderValue::new("curl/7.64.1"));
            expected_headers.insert(HeaderName::Accept, HeaderValue::new("*/*"));
            Headers::new(expected_headers)
        };

        let headers =
            Headers::try_from("Host: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*")
                .expect("Test string is valid");

        assert_eq!(headers, expected_headers);
    }
}
