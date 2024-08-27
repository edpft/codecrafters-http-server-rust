use crate::{error::Error, version::Version};

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct RequestLine {
    http_method: HttpMethod,
    request_target: String,
    http_version: Version,
}

impl RequestLine {
    pub fn new(
        http_method: HttpMethod,
        request_target: impl Into<String>,
        http_version: Version,
    ) -> RequestLine {
        let request_target = request_target.into();
        Self {
            http_method,
            request_target,
            http_version,
        }
    }

    pub fn target(&self) -> &str {
        &self.request_target
    }
}

impl<'a> TryFrom<&'a str> for RequestLine {
    type Error = Error<'a>;

    fn try_from(string: &'a str) -> Result<Self, Self::Error> {
        let mut parts = string.split_whitespace();
        let http_method = match parts.next() {
            None => {
                let error = Error::NotEnoughRequestLineParts(0);
                Err(error)
            }
            Some(method) => HttpMethod::try_from(method),
        }?;
        let request_target = match parts.next() {
            None => {
                let error = Error::NotEnoughRequestLineParts(1);
                Err(error)
            }
            Some(target) => Ok(target),
        }?;
        let http_version = match parts.next() {
            None => {
                let error = Error::NotEnoughRequestLineParts(2);
                Err(error)
            }
            Some(version) => Version::try_from(version).map_err(|_| Error::InvalidVersion(version)),
        }?;
        // TODO check if there are any more parts
        let request_line = Self::new(http_method, request_target, http_version);
        Ok(request_line)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub enum HttpMethod {
    #[default]
    Get,
}

impl<'a> TryFrom<&'a str> for HttpMethod {
    type Error = Error<'a>;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            other => Err(Error::InvalidMethod(other)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index_http_1_1() {
        let expected_request_line = RequestLine::new(
            HttpMethod::Get,
            String::from("/index.html"),
            Version::OnePointOne,
        );
        let request_line = RequestLine::try_from("GET /index.html HTTP/1.1")
            .expect("Valid request line is parsed successfully");
        assert_eq!(request_line, expected_request_line);
    }
}
