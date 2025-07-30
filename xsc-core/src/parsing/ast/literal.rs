use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
}

impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Literal::Int(val1), Literal::Int(val2))     => val1 == val2,
            (Literal::Float(val1), Literal::Float(val2)) => val1 == val2,
            (Literal::Bool(val1), Literal::Bool(val2))   => val1 == val2,
            (Literal::Str(val1), Literal::Str(val2))     => val1 == val2,
            _                                            => false
        }
    }
}

impl Eq for Literal {}

impl Hash for Literal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Literal::Int(val)   => val.hash(state),
            Literal::Float(val) => format!("π{:}Σ", val).hash(state),
            Literal::Bool(val)  => val.hash(state),
            Literal::Str(val)   => val.hash(state),
        }
        let _ = state.finish();
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Int(v) => { write!(f, "{}", v) }
            Literal::Float(v) => { write!(f, "{}", v) }
            Literal::Bool(v) => { write!(f, "{}", v) }
            Literal::Str(v) => { write!(f, "{}", v) }
        }
    }
}

impl Literal {
    pub fn str(val: &str) -> Self {
        Literal::Str(val.to_string())
    }
}