#[cfg(not(feature = "lsp"))]
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

use chumsky::input::Input;
use chumsky::Parser;

use crate::parsing::lexer::{lexer, Token};
use crate::parsing::parser::parser;
use crate::r#static::info::{AstCacheRef, Error, ParseError, SrcCacheRef, TypeEnv};
use crate::r#static::type_check::xs_tc;
use crate::utils::{pop};

pub fn gen_errs_from_path(
    path: &PathBuf,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
    src_cache: SrcCacheRef,
) -> Result<(), Vec<Error>> {
    #[cfg(feature = "lsp")]
    let result = match src_cache.get(path) {
        Some(entry) => {
            Ok(entry.value().1.to_string())
        }
        None => fs::read_to_string(path),
    };

    #[cfg(not(feature = "lsp"))]
    let result = match src_cache.get(path) {
        Some(src) => {
            Ok(Cow::Borrowed(src))
        }
        None => fs::read_to_string(path).map(Cow::Owned),
    };

    let src = match result {
        Ok(src) => { src }
        Err(err) => {
            let path_str = path.display();
            return Err(vec![Error::FileErr(path.clone(), format!("Failed to read path '{path_str}', details: {err}"))])
        }
    };

    gen_errs_from_src(path, &src, type_env, ast_cache, src_cache)
}

pub fn gen_errs_from_src(
    path: &PathBuf,
    src: &str,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
    src_cache: SrcCacheRef,
) -> Result<(), Vec<Error>> {
    let (tokens, errs) = lexer()
        .parse(src)
        .into_output_errors();
    
    let hash = blake3::hash(src.as_bytes());
    if let Some((prev_hash, (ast, comments))) = pop(ast_cache, path) {
        let Some(prev_hash) = prev_hash else {
            return Err(vec![Error::FileErr(
                path.clone(), format!("Circular dependency detected while parsing '{}'", path.display())
            )])
        };
        ast_cache.insert(path.clone(), (None, (vec![], vec![])));
        if hash == prev_hash {
            let r = xs_tc(path, &ast, type_env, ast_cache, src_cache, &comments);
            ast_cache.insert(path.clone(), (Some(hash), (ast, comments)));
            return r
        }
    };
    
    let Some(tokens) = tokens else {
        ast_cache.insert(path.clone(), (Some(hash), (vec![], vec![])));
        return Err(vec![Error::parse_errs(
            path,
            errs.iter()
                .map(ParseError::lex_err)
                .collect()
        )]);
    };

    let (tokens, comments) = tokens.into_iter()
        .partition::<Vec<_>, _>(|tok| !tok.0.is_comment());
    
    let comments = comments.into_iter()
        .map(|(val, span)| match val {
            Token::Comment(msg) => (msg, span),
            _ => unreachable!(),
        }).collect();
    
    let (ast, errs) = parser()
        .map_with(|ast, e| (ast, e.span()))
        .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
        .into_output_errors();

    let Some((ast, _span)) = ast else {
        ast_cache.insert(path.clone(), (Some(hash), (vec![], vec![])));
        return Err(vec![Error::parse_errs(
            path,
            errs.iter()
                .map(ParseError::parse_err)
                .collect()
        )]);
    };

    ast_cache.insert(path.clone(), (None, (vec![], vec![])));
    let r = xs_tc(path, &ast, type_env, ast_cache, src_cache, &comments);
    ast_cache.insert(path.clone(), (Some(hash), (ast, comments)));
    r
}