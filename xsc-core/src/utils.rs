use std::collections::{HashSet};
use std::hash::Hash;

use crate::r#static::info::{AstMapRef, WarningKind};

pub fn warnings_from_str(ignores: &str) -> Result<HashSet<u32>, &str> {
    ignores
        .split(|c| c == ',' || c == ' ')
        .map(str::trim)
        .filter(|str_| !str_.is_empty())
        .map(|str_| {
            match WarningKind::from_name(str_) {
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
