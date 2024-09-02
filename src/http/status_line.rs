use std::fmt;

use crate::version::Version;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct StatusLine {
    http_version: Version,
    status: Status,
}

impl StatusLine {
    pub fn make_http_1_1_status_line(status: Status) -> Self {
        let http_version = Version::default();
        Self {
            http_version,
            status,
        }
    }

    pub fn ok() -> Self {
        let http_version = Version::default();
        let status = Status::Ok;
        Self {
            http_version,
            status,
        }
    }

    pub fn not_found() -> Self {
        let http_version = Version::default();
        let status = Status::NotFound;
        Self {
            http_version,
            status,
        }
    }

    pub fn internal_server_error() -> Self {
        let http_version = Version::default();
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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum Status {
    #[default]
    Ok,
    NotFound,
    InternalServerError,
    Created,
    BadRequest,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok => write!(f, "200 OK"),
            Self::NotFound => write!(f, "404 Not Found"),
            Self::InternalServerError => write!(f, "500 Internal Server Error"),
            Self::Created => write!(f, "201 Created"),
            Self::BadRequest => write!(f, "400 Bad Request"),
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
