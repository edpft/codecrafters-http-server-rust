use std::{collections::HashMap, fmt};

use crate::error::RequestError;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Headers(HashMap<HeaderName, HeaderValue>);

impl Headers {
    pub fn new(headers: HashMap<HeaderName, HeaderValue>) -> Self {
        Self(headers)
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

impl<'a> Iterator for &'a Headers {
    type Item = (&'a HeaderName, &'a HeaderValue);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.iter().next()
    }
}

impl IntoIterator for Headers {
    type Item = (HeaderName, HeaderValue);
    type IntoIter = std::collections::hash_map::IntoIter<HeaderName, HeaderValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

impl<'a> TryFrom<&'a str> for Headers {
    type Error = RequestError<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut headers: HashMap<HeaderName, HeaderValue> = HashMap::default();
        for header_name_value_pair in value.split("\r\n") {
            if header_name_value_pair.is_empty() {
                break;
            };
            let Some(mid) = header_name_value_pair.find(':') else {
                let error = RequestError::InvalidHeaderNameValuePair(header_name_value_pair);
                return Err(error);
            };
            let (key, value) = header_name_value_pair.split_at(mid);
            let header_name = HeaderName::try_from(key)?;
            let trimmed_header_value = value
                .strip_prefix(':')
                .expect("value must contained a ':' because we found it above")
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
    ContentType,
    ContentLength,
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

impl<'a> TryFrom<&'a str> for HeaderName {
    type Error = RequestError<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "Host" => Ok(Self::Host),
            "User-Agent" => Ok(Self::UserAgent),
            "Accept" => Ok(Self::Accept),
            "Content-Type" => Ok(Self::ContentType),
            "Content-Length" => Ok(Self::ContentLength),
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
    use super::*;

    #[test]
    fn serialize_deserialize_valid_request_headers() {
        let headers = Headers::default()
            .set_host("localhost:4221")
            .set_user_agent("curl/7.64.1")
            .set_accept("*/*");

        let serialized_headers = headers.to_string();

        let deserialized_headers =
            Headers::try_from(serialized_headers.as_str()).expect("Serialized headers are valid");

        assert_eq!(headers, deserialized_headers);
    }

    #[test]
    fn serialize_deserialize_valid_response_headers() {
        let headers = Headers::default()
            .set_content_type(ContentType::Text)
            .set_content_length(3);

        let serialized_headers = headers.to_string();

        let deserialized_headers =
            Headers::try_from(serialized_headers.as_str()).expect("Serialized headers are valid");

        assert_eq!(headers, deserialized_headers);
    }
}
