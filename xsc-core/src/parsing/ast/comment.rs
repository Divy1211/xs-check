use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct Comment(pub String);

// all comments are equal for the parser.
impl Hash for Comment {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_i32(6);
        let _ = state.finish();
    }
}

impl PartialEq for Comment {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Eq for Comment {}

impl Comment {
    pub fn new(cmt: &str) -> Self {
        Comment(cmt.to_string())
    }
}
