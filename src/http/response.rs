use std::fmt;

use crate::{
    body::Body,
    headers::Headers,
    response_builder::ResponseBuilder,
    status_line::{Status, StatusLine},
};

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct Response {
    status_line: StatusLine,
    headers: Headers,
    body: Option<Body>,
}

impl Response {
    pub fn new(status_line: StatusLine, headers: Headers, body: Option<Body>) -> Self {
        Response {
            status_line,
            headers,
            body,
        }
    }

    pub fn ok() -> ResponseBuilder {
        ResponseBuilder::default().set_status(Status::Ok)
    }

    pub fn created() -> ResponseBuilder {
        ResponseBuilder::default().set_status(Status::Created)
    }

    pub fn not_found() -> ResponseBuilder {
        ResponseBuilder::default().set_status(Status::NotFound)
    }

    pub fn internal_server_error() -> ResponseBuilder {
        ResponseBuilder::default().set_status(Status::InternalServerError)
    }

    pub fn bad_request() -> ResponseBuilder {
        ResponseBuilder::default().set_status(Status::BadRequest)
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.body {
            None => write!(f, "{}{}\r\n", self.status_line, self.headers),
            Some(body) => write!(f, "{}{}\r\n{}", self.status_line, self.headers, body),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::headers::ContentType;

    use super::*;

    #[test]
    fn ok_response() {
        let default_response = Response::ok().build();
        assert_eq!(
            default_response.to_string(),
            String::from("HTTP/1.1 200 OK\r\n\r\n")
        );
    }

    #[test]
    fn not_found_response() {
        let default_response = Response::not_found().build();
        assert_eq!(
            default_response.to_string(),
            String::from("HTTP/1.1 404 Not Found\r\n\r\n")
        );
    }

    #[test]
    fn plain_text_response() {
        let expected_response = Response::new(
            StatusLine::ok(),
            Headers::default()
                .set_content_type(ContentType::Text)
                .set_content_length(3),
            Some(Body::from("abc")),
        );
        let response = Response::ok().set_body("abc").build();
        assert_eq!(response, expected_response,);
    }
}
