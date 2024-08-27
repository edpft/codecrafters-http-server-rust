use std::fmt;

use crate::{body::Body, headers::Headers, status_line::StatusLine};

#[derive(Clone, Debug, Default)]
pub struct Response {
    status_line: StatusLine,
    headers: Headers,
    body: Option<Body>,
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.body {
            None => write!(f, "{}{}", self.status_line, self.headers),
            Some(body) => write!(f, "{}{}{}", self.status_line, self.headers, body),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_response() {
        let default_response = Response::default();
        assert_eq!(
            default_response.to_string(),
            String::from("HTTP/1.1 200 OK\r\n\r\n")
        );
    }
}
