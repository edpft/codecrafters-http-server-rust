use std::fmt;

use crate::{
    body::Body, error::RequestError, headers::Headers, request_line::RequestLine,
    status_line::StatusLine,
};

#[derive(Clone, Debug)]
pub struct Response {
    status_line: StatusLine,
    headers: Headers,
    body: Option<Body>,
}

impl Response {
    pub fn ok() -> Response {
        let status_line = StatusLine::ok();
        let headers = Headers::default();
        let body = None;
        Self {
            status_line,
            headers,
            body,
        }
    }

    pub fn not_found() -> Response {
        let status_line = StatusLine::not_found();
        let headers = Headers::default();
        let body = None;
        Self {
            status_line,
            headers,
            body,
        }
    }

    pub fn internal_server_error() -> Response {
        let status_line = StatusLine::internal_server_error();
        let headers = Headers::default();
        let body = None;
        Self {
            status_line,
            headers,
            body,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.body {
            None => write!(f, "{}{}", self.status_line, self.headers),
            Some(body) => write!(f, "{}{}{}", self.status_line, self.headers, body),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Request {
    request_line: RequestLine,
    headers: Headers,
    body: Option<Body>,
}

impl Request {
    pub fn new(request_line: RequestLine, headers: Headers, body: Option<Body>) -> Self {
        Self {
            request_line,
            headers,
            body,
        }
    }

    pub fn target(&self) -> &str {
        self.request_line.target()
    }
}

impl<'a> TryFrom<&'a [u8]> for Request {
    type Error = RequestError<'a>;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Self::Error> {
        let string_ref: &str =
            std::str::from_utf8(bytes).map_err(|_| RequestError::Parsing(bytes))?;
        Request::try_from(string_ref)
    }
}

impl<'a> TryFrom<&'a str> for Request {
    type Error = RequestError<'a>;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        let Some((request_line_string, remainder)) = string.split_once("\r\n") else {
            let error = RequestError::NotEnoughRequestParts(0);
            return Err(error);
        };

        let request_line = RequestLine::try_from(request_line_string)?;

        let Some((headers_string, _body_string)) = remainder.split_once("\r\n\r\n") else {
            let error = RequestError::NotEnoughRequestParts(1);
            return Err(error);
        };

        let headers = Headers::try_from(headers_string)?;

        let request = Self::new(request_line, headers, None);
        Ok(request)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        headers::{HeaderName, HeaderValue},
        request_line::HttpMethod,
        version::HttpVersion,
    };

    use super::*;

    #[test]
    fn ok_response() {
        let default_response = Response::ok();
        assert_eq!(
            default_response.to_string(),
            String::from("HTTP/1.1 200 OK\r\n\r\n")
        );
    }

    #[test]
    fn not_found_response() {
        let default_response = Response::not_found();
        assert_eq!(
            default_response.to_string(),
            String::from("HTTP/1.1 404 Not Found\r\n\r\n")
        );
    }

    #[test]
    fn parse_valid_request_without_body() {
        let expected_headers = {
            let mut expected_headers: HashMap<HeaderName, HeaderValue> = HashMap::default();
            expected_headers.insert(HeaderName::Host, HeaderValue::new("localhost:4221"));
            expected_headers.insert(HeaderName::UserAgent, HeaderValue::new("curl/7.64.1"));
            expected_headers.insert(HeaderName::Accept, HeaderValue::new("*/*"));
            Headers::new(expected_headers)
        };

        let expected_request = Request::new(
            RequestLine::new(
                HttpMethod::Get,
                String::from("/index.html"),
                HttpVersion::OnePointOne,
            ),
            expected_headers,
            None,
        );

        let request = Request::try_from("GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n").expect("Test string is valid");

        assert_eq!(request, expected_request);
    }
}
