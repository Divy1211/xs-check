use std::path::PathBuf;

use crate::parsing::span::Span;

#[derive(Clone, Debug)]
pub struct SrcLoc {
    pub file_path: PathBuf,
    pub span: Span,
}

impl SrcLoc {
    pub fn from(file_path: &PathBuf, span: &Span) -> Self {
        Self {
            file_path: file_path.clone(),
            span: span.clone(),
        }
    }
}

impl Default for SrcLoc {
    fn default() -> Self {
        Self {
            file_path: Default::default(),
            span: Span::new(0, 0)
        }
    }
}