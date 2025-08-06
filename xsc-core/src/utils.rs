use std::io::Write;
use std::collections::{HashSet};
use std::fs::OpenOptions;
use std::hash::Hash;
use crate::r#static::info::{AstMapRef, WarningKind};

pub fn warnings_from_str(ignores: &str) -> Result<HashSet<u32>, &str> {
    ignores
        .split(|c| c == ',' || c == ' ')
        .map(str::trim)
        .filter(|str_| !str_.is_empty())
        .map(|str_| {
            match WarningKind::from_str(str_) {
                None => { Err(str_) }
                Some(kind) => { Ok(kind.as_u32()) }
            }
        }).collect()
}

pub fn pop<K: Eq + Hash, V>(cache: AstMapRef<K, V>, path: &K) -> Option<V> {
    #[cfg(feature = "lsp")]
    return cache.remove(path).map(|(_path, entry)| entry);

    #[cfg(not(feature = "lsp"))]
    return cache.remove(path);
}

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
