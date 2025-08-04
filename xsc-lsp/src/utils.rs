use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use tower_lsp::lsp_types::Url;

#[allow(dead_code)]
pub fn log(message: &str) {
    let path = r"C:\Users\Divy\My Stuff\web dev\VSCE\aoe2xsscripting\server\xsc-lsp.log";
    
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = writeln!(file, "{}", message);
    }
}

pub fn path_from_uri(uri: &Url) -> PathBuf {
    match uri.to_file_path() {
        Ok(path) => path,
        Err(_) => {
            // unsaved files don't have paths, so just use the untitled filename given by VSC
            PathBuf::from(uri.to_string())
        }
    }
}
