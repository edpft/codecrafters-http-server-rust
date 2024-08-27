use std::fmt;

use crate::version::HttpVersion;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StatusLine {
    http_version: HttpVersion,
    status: Status,
}

impl StatusLine {
    pub fn ok() -> Self {
        let http_version = HttpVersion::default();
        let status = Status::Ok;
        Self {
            http_version,
            status,
        }
    }

    pub fn not_found() -> Self {
        let http_version = HttpVersion::default();
        let status = Status::NotFound;
        Self {
            http_version,
            status,
        }
    }

    pub fn internal_server_error() -> Self {
        let http_version = HttpVersion::default();
        let status = Status::InternalServerError;
        Self {
            http_version,
            status,
        }
    }
}

impl fmt::Display for StatusLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}\r\n", self.http_version, self.status)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Status {
    Ok,
    NotFound,
    InternalServerError,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_status_line() {
        let default_status_line = StatusLine::ok();
        assert_eq!(
            default_status_line.to_string(),
            String::from("HTTP/1.1 200 OK\r\n")
        );
    }
}
