use std::collections::HashSet;
use std::path::PathBuf;
use crate::config::flavour::Flavour;

#[allow(dead_code)]
#[derive(Debug)]
pub struct ExtConfig {
    pub ignores: HashSet<u32>,
    pub extra_prelude_path: Option<PathBuf>,
    pub include_dirs: Vec<PathBuf>,
    pub flavour: Flavour
}
