use std::path::PathBuf;

use xsc_core::r#static::info::{gen_errs_from_path, gen_errs_from_src, AstCache, AstMap, Error, TypeEnv};

use crate::cli::parse_args;
use crate::fmt::{print_parse_errs, print_xs_errs};

mod cli;
mod fmt;

fn main() {
    let (filepath, ignores, extra_prelude_path, include_dirs) = match parse_args() {
        Some(filepath) => { filepath }
        None => { return; },
    };
    
    let mut type_env= TypeEnv::new(include_dirs);
    let mut ast_cache = AstMap::new();
    
    let prelude_path = PathBuf::from(r"prelude.xs");
    let prelude = include_str!(r"../../xsc-core/prelude.xs");

    gen_errs_from_src(&prelude_path, prelude, &mut type_env, &mut ast_cache).expect("Prelude can't produce parse errors");

    let mut has_errors = false;
    if let Some(extra_prelude_path) = extra_prelude_path {
        let new_errs = check_file(&extra_prelude_path, &mut type_env, &mut ast_cache);
        has_errors = new_errs;
    }
    let new_errs = check_file(&filepath, &mut type_env, &mut ast_cache);
    has_errors =  has_errors || new_errs;

    for (filepath, errs) in type_env.errs() {
        if errs.len() == 0 {
            continue;
        } else if filepath == &prelude_path {
            panic!("Prelude can't produce errors")
        }
        let new_errs = print_xs_errs(filepath, errs, &ignores);
        has_errors = has_errors || new_errs;
    }

    if !has_errors {
        println!(
            "No errors found in file '{}'! Your code is free of the pitfalls of XS' quirks =)",
            filepath.display()
        );
    }
    println!("Finished analysing file '{}'.", filepath.display());
}

fn check_file(filepath: &PathBuf, type_env: &mut TypeEnv, ast_cache: &mut AstCache) -> bool {
    let mut has_errors = false;
    if let Err(errs) = gen_errs_from_path(&filepath, type_env, ast_cache) {
        has_errors = true;
        for err in errs {
            match err {
                Error::FileErr(_path, msg) => {
                    println!("{}", msg);
                }
                Error::ParseErrs { path, errs } => {
                    print_parse_errs(&path, &errs);
                }
            }
        }
    }
    has_errors
}
