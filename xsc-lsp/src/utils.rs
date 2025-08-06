use std::path::PathBuf;
use dunce::canonicalize;
use tower_lsp::lsp_types::Url;

pub fn path_from_uri(uri: &Url) -> PathBuf {
    match uri.to_file_path() {
        Ok(path) => canonicalize(path).expect("Infallible"),
        Err(_) => {
            // unsaved files don't have paths, so just use the untitled filename given by VSC
            PathBuf::from(uri.to_string())
        }
    }
}
