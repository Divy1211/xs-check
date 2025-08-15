#[cfg(feature = "lsp")]
mod types {
    use dashmap::DashMap;
    use ropey::Rope;
    use tower_lsp::lsp_types::Url;

    use crate::r#static::info::{AstCache, SrcCache};

    pub type AstMap<K, V> = DashMap<K, V>;

    pub type AstMapRef<'a, K, V> = &'a AstMap<K, V>;

    pub type AstCacheRef<'a> = &'a AstCache;

    pub type SrcInfo = (Url, Rope);

    pub type SrcCacheRef<'a> = &'a SrcCache;
}

#[cfg(not(feature = "lsp"))]
mod types {
    use crate::r#static::info::{AstCache, SrcCache};

    pub type AstMap<K, V> = std::collections::HashMap<K, V>;

    pub type AstMapRef<'a, K, V> = &'a mut AstMap<K, V>;

    pub type AstCacheRef<'a> = &'a mut AstCache;

    pub type SrcInfo = String;

    pub type SrcCacheRef<'a> = &'a SrcCache;
}

use std::path::PathBuf;
use blake3::Hash;

use crate::parsing::ast::AstNode;
use crate::parsing::span::Spanned;

pub use types::*;

pub type AstInfo = (Option<Hash>, (Vec<Spanned<AstNode>>, Vec<Spanned<String>>));

pub type AstCache = AstMap<PathBuf, AstInfo>;

pub type SrcCache = AstMap<PathBuf, SrcInfo>;