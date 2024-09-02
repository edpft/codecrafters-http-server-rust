use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Body {
    PlainText(String),
    OctetStream(Vec<u8>),
}

impl Body {
    pub fn len(&self) -> usize {
        match self {
            Body::PlainText(string) => string.len(),
            Body::OctetStream(bytes) => bytes.len(),
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        match self {
            Self::OctetStream(bytes) => bytes,
            Self::PlainText(string) => string.as_bytes(),
        }
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Body::PlainText(string) => write!(f, "{string}"),
            Body::OctetStream(bytes) => match String::from_utf8(bytes.to_owned()) {
                Ok(string) => write!(f, "{string}"),
                Err(_) => write!(f, "{bytes:?}"),
            },
        }
    }
}

impl From<&str> for Body {
    fn from(body: &str) -> Self {
        let body = body.to_string();
        Self::from(body)
    }
}

impl From<String> for Body {
    fn from(body: String) -> Self {
        Self::PlainText(body)
    }
}

impl From<Vec<u8>> for Body {
    fn from(body: Vec<u8>) -> Self {
        Self::OctetStream(body)
    }
}
