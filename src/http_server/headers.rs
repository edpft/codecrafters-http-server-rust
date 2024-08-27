use std::{collections::HashSet, fmt};

#[derive(Clone, Debug, Default)]
pub struct Headers(HashSet<Headers>);

impl fmt::Display for Headers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\r\n")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Header {}
