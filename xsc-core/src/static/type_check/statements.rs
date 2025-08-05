use std::path::PathBuf;

use crate::parsing::ast::AstNode;
use crate::parsing::span::Spanned;
use crate::r#static::type_check::statement::xs_tc_stmt;
use crate::r#static::info::{AstCacheRef, Error, TypeEnv};
use crate::r#static::type_check::util::combine_results;

pub fn xs_tc(
    path: &PathBuf,
    stmts: &Vec<Spanned<AstNode>>,
    type_env: &mut TypeEnv,
    ast_cache: AstCacheRef,
) -> Result<(), Vec<Error>> {
    combine_results(stmts.iter()
        .map(|stmt| {
            xs_tc_stmt(
                path, stmt, type_env, ast_cache,
                true, false, false,
        )})
    )
}
