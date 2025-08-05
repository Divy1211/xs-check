use std::path::PathBuf;
use std::fs;

use chumsky::input::Input;
use chumsky::Parser;
use crate::parsing::lexer::{lexer, Token};
use crate::parsing::parser::parser;
use crate::parsing::ast::AstNode;
use crate::parsing::span::Spanned;
use crate::r#static::info::{Error, ParseError, TypeEnv};
use crate::r#static::type_check::xs_tc;

#[cfg(feature = "lsp")]
pub type AstMap<K, V> = dashmap::DashMap<K, V>;

#[cfg(not(feature = "lsp"))]
pub type AstMap<K, V> = std::collections::HashMap<K, V>;

pub type AstCache = AstMap<PathBuf, Vec<Spanned<AstNode>>>;

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
            let path = path.display();
            return Err(vec![Error::FileErr(format!("Failed to read path {path}, details: {err}"))])
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

    let Some(mut tokens) = tokens else {
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
        return Err(vec![Error::parse_errs(
            path,
            errs.iter()
                .map(ParseError::parse_err)
                .collect()
        )]);
    };

    let r = xs_tc(path, &ast, type_env, ast_cache);
    ast_cache.insert(path.clone(), ast);
    r
}