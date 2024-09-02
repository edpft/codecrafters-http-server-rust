use nom::{bytes::complete::take, sequence::Tuple, IResult};

use crate::{
    body::Body,
    headers::{HeaderName, Headers},
    method::Method,
    parsing_utils,
    path::Path,
    version::Version,
};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Request {
    method: Method,
    target: Path,
    version: Version,
    headers: Headers,
    body: Option<Body>,
}

impl Request {
    pub fn new(
        method: Method,
        target: Path,
        version: Version,
        headers: Headers,
        body: Option<Body>,
    ) -> Self {
        Self {
            method,
            target,
            version,
            headers,
            body,
        }
    }

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remainder, (request_line, headers)) =
            (RequestLine::parse, Headers::parse).parse(bytes)?;

        let body = match headers.get(&HeaderName::ContentLength) {
            None => None,
            Some(header_value) => {
                let content_length = header_value
                    .as_usize()
                    .expect("The value of Content-Length should be an integer");
                let (_, body) = take(content_length)(remainder)?;
                let body = body.to_owned();
                let body = Body::OctetStream(body);
                Some(body)
            }
        };

        let request = Self::new(
            request_line.method,
            request_line.target,
            request_line.version,
            headers,
            body,
        );
        Ok((remainder, request))
    }

    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    pub fn target(&self) -> &Path {
        &self.target
    }

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn body(&self) -> Option<&Body> {
        self.body.as_ref()
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
struct RequestLine {
    pub method: Method,
    pub target: Path,
    pub version: Version,
}

impl RequestLine {
    pub fn new(method: Method, target: Path, version: Version) -> Self {
        Self {
            method,
            target,
            version,
        }
    }
    pub fn parse(bytes: &[u8]) -> IResult<&[u8], RequestLine> {
        let (remainder, (method, _, target, _, version, _)) = (
            Method::parse,
            parsing_utils::space,
            Path::parse,
            parsing_utils::space,
            Version::parse,
            parsing_utils::crlf,
        )
            .parse(bytes)?;
        let request_line = RequestLine::new(method, target, version);
        Ok((remainder, request_line))
    }
}

#[cfg(test)]
mod tests {
    use std::str;

    use crate::{method::Method, version::Version};

    use super::*;

    fn make_expected_headers() -> Headers {
        Headers::default()
            .set_host("localhost:4221")
            .set_user_agent("curl/7.64.1")
            .set_accept("*/*")
    }

    #[test]
    fn parse_request_line() {
        let bytes = b"GET / HTTP/1.1\r\n\r\n";
        let expected_headers = Headers::default();
        let expected_request = Request::new(
            Method::Get,
            Path::new("/"),
            Version::OnePointOne,
            expected_headers,
            None,
        );

        let (remainder, request) = Request::parse(bytes).unwrap_or_else(|_| {
            panic!(
                "Cannot parse bytes:\n{}",
                str::from_utf8(bytes).expect("Bytes are valid UTF8")
            )
        });

        assert!(remainder.is_empty());
        assert_eq!(request, expected_request);
    }

    #[test]
    fn parse_request_line_and_headers() {
        let bytes = b"GET /index.html HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n";

        let expected_headers = make_expected_headers();
        let expected_request = Request::new(
            Method::Get,
            Path::new("/index.html"),
            Version::OnePointOne,
            expected_headers,
            None,
        );

        let (remainder, request) = Request::parse(bytes).unwrap_or_else(|_| {
            panic!(
                "Cannot parse bytes:\n{}",
                str::from_utf8(bytes).expect("Bytes are valid UTF8")
            )
        });

        assert!(remainder.is_empty());
        assert_eq!(request, expected_request);
    }
}
