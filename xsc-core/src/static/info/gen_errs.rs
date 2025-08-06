use std::path::PathBuf;
use std::fs;
use blake3::Hash;
use chumsky::input::Input;
use chumsky::Parser;
use crate::parsing::lexer::{lexer, Token};
use crate::parsing::parser::parser;
use crate::parsing::ast::AstNode;
use crate::parsing::span::Spanned;
use crate::r#static::info::{Error, ParseError, TypeEnv};
use crate::r#static::type_check::xs_tc;
use crate::utils::{pop};

#[cfg(feature = "lsp")]
pub type AstMap<K, V> = dashmap::DashMap<K, V>;

#[cfg(not(feature = "lsp"))]
pub type AstMap<K, V> = std::collections::HashMap<K, V>;

pub type AstInfo = (Option<Hash>, Vec<Spanned<AstNode>>);

pub type AstCache = AstMap<PathBuf, AstInfo>;

#[cfg(feature = "lsp")]
pub type AstCacheRef<'a> = &'a AstCache;

#[cfg(not(feature = "lsp"))]
pub type AstCacheRef<'a> = &'a mut AstCache;

pub fn gen_errs_from_path(
    path: &PathBuf,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
) -> Result<(), Vec<Error>> {
    let src = match fs::read_to_string(&path) {
        Ok(src) => {src}
        Err(err) => {
            let path_str = path.display();
            return Err(vec![Error::FileErr(path.clone(), format!("Failed to read path '{path_str}', details: {err}"))])
        }
    };

    gen_errs_from_src(path, &src, type_env, ast_cache)
}

pub fn gen_errs_from_src(
    path: &PathBuf,
    src: &str,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
) -> Result<(), Vec<Error>> {
    let (tokens, errs) = lexer()
        .parse(src)
        .into_output_errors();
    
    let hash = blake3::hash(src.as_bytes());
    if let Some((prev_hash, ast)) = pop(ast_cache, path) {
        let Some(prev_hash) = prev_hash else {
            return Err(vec![Error::FileErr(
                path.clone(), format!("Circular dependency detected while parsing '{}'", path.display())
            )])
        };
        ast_cache.insert(path.clone(), (None, vec![]));
        if hash == prev_hash {
            let r = xs_tc(path, &ast, type_env, ast_cache);
            ast_cache.insert(path.clone(), (Some(hash), ast));
            return r;
        }
    };
    
    let Some(mut tokens) = tokens else {
        ast_cache.insert(path.clone(), (Some(hash), vec![]));
        return Err(vec![Error::parse_errs(
            path,
            errs.iter()
                .map(ParseError::lex_err)
                .collect()
        )]);
    };

    tokens = tokens.into_iter()
        .filter(|tok| match tok { (Token::Comment(_), _) => { false }, _ => { true } })
        .collect();

    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        ast_cache.insert(path.clone(), (Some(hash), vec![]));
        return Err(vec![Error::parse_errs(
            path,
            errs.iter()
                .map(ParseError::parse_err)
                .collect()
        )]);
    };

    ast_cache.insert(path.clone(), (None, vec![]));
    let r = xs_tc(path, &ast, type_env, ast_cache);
    ast_cache.insert(path.clone(), (Some(hash), ast));
    r
}