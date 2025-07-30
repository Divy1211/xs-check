use std::path::PathBuf;

use crate::r#static::info::ParseError;

#[derive(Debug)]
pub enum Error {
    FileErr(String),
    ParseErrs { path: PathBuf, errs: Vec<ParseError> },
}

impl Error {
    pub fn parse_errs(path: &PathBuf, errs: Vec<ParseError>) -> Self {
        Self::ParseErrs { path: path.clone(), errs }
    }
}

