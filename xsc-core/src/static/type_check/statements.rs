use std::path::PathBuf;

use crate::parsing::ast::AstNode;
use crate::parsing::span::Spanned;
use crate::r#static::type_check::statement::xs_tc_stmt;
use crate::r#static::info::{AstCacheRef, Error, SrcCacheRef, TypeEnv};
use crate::r#static::type_check::util::combine_results;

pub fn xs_tc(
    path: &PathBuf,
    stmts: &Vec<Spanned<AstNode>>,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
    src_cache: SrcCacheRef,
    comments: &Vec<Spanned<String>>,
) -> Result<(), Vec<Error>> {
    let mut comment_pos = 0;
    combine_results(stmts.iter()
        .map(|stmt| {
            xs_tc_stmt(
                path, stmt, type_env, ast_cache, src_cache, comments, &mut comment_pos,
                true, false, false,
        )})
    )
}
