use std::collections::{HashSet};
use std::path::PathBuf;
use blake3::Hash;
use crate::r#static::info::{AstCacheRef, WarningKind};

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

pub fn is_unchanged(ast_cache: AstCacheRef, path: &PathBuf, hash: Hash) -> bool {
    #[cfg(feature = "lsp")]
    return ast_cache.get(path).map_or(false, |cache| {
        cache.value().0 == hash
    });

    #[cfg(not(feature = "lsp"))]
    return ast_cache.get(path).map_or(false, |(prev_hash, _ast)| { hash == *prev_hash });
}