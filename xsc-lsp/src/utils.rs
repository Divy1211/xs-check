use std::path::PathBuf;
use dunce::canonicalize;
use tower_lsp::lsp_types::Url;

pub fn path_from_uri(uri: &Url) -> PathBuf {
    match uri.to_file_path() {
        Ok(path) if path.exists() => canonicalize(path).expect("Infallible"),
        _ => {
            // unsaved files don't have paths, so just use the untitled filename given by the client
            PathBuf::from(uri.to_string())
        }
    }
}
