use std::collections::{HashSet};
use std::path::PathBuf;
use crate::r#static::info::{AstCacheRef, AstInfo, WarningKind};

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

pub fn pop(ast_cache: AstCacheRef, path: &PathBuf) -> Option<AstInfo> {
    #[cfg(feature = "lsp")]
    return ast_cache.remove(path).map(|(_path, entry)| entry);

    #[cfg(not(feature = "lsp"))]
    return ast_cache.remove(path);
}