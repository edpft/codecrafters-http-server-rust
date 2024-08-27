use crate::{body::Body, error::RequestError, headers::Headers, request_line::RequestLine};

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

    pub fn headers(&self) -> &Headers {
        &self.headers
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

    use crate::{request_line::HttpMethod, version::HttpVersion};

    use super::*;

    fn make_expected_headers() -> Headers {
        Headers::default()
            .set_host("localhost:4221")
            .set_user_agent("curl/7.64.1")
            .set_accept("*/*")
    }

    #[test]
    fn parse_valid_index_request_without_body() {
        let expected_headers = make_expected_headers();

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

    #[test]
    fn parse_valid_echo_request_without_body() {
        let expected_headers = make_expected_headers();

        let expected_request = Request::new(
            RequestLine::new(
                HttpMethod::Get,
                String::from("/echo/abc"),
                HttpVersion::OnePointOne,
            ),
            expected_headers,
            None,
        );

        let request = Request::try_from("GET /echo/abc HTTP/1.1\r\nHost: localhost:4221\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n").expect("Test string is valid");

        assert_eq!(request, expected_request);
    }
}
