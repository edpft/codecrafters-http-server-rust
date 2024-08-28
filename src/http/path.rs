use nom::{bytes, IResult};
use std::str;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Path(String);

impl Path {
    pub fn new(path: impl Into<String>) -> Self {
        let path = path.into();
        Self(path)
    }

    pub fn parse(bytes: &[u8]) -> IResult<&[u8], Self> {
        let (remainder, path_bytes) = bytes::complete::take_while1(|byte| byte != b' ')(bytes)?;
        let path_string = str::from_utf8(path_bytes).expect("Path string is valid UTF8");
        let path = Self::new(path_string);
        Ok((remainder, path))
    }

    pub fn starts_with(&self, pattern: &str) -> bool {
        self.0.starts_with(pattern)
    }

    pub fn strip_prefix(&self, prefix: &str) -> Option<Self> {
        self.0.strip_prefix(prefix).map(Self::new)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PartialEq<str> for Path {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}
