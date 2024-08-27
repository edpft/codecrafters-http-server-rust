use std::fmt;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Body(String);

impl Body {
    pub fn new(body: impl Into<String>) -> Self {
        let body = body.into();
        Self(body)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for Body {
    fn from(body: &str) -> Self {
        Self::new(body)
    }
}
