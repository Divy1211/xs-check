use std::fmt::{Display, Formatter};
use std::hash::{Hash};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Identifier(pub String);

impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_string())
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}