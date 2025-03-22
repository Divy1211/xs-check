use std::path::PathBuf;
use crate::cli::parse_args;
use crate::lint::{gen_errs_from_path, gen_errs_from_src, print_parse_errs, print_xs_errs};
use crate::r#static::info::{Error, TypeEnv};

pub mod parsing;
mod cli;
pub mod lint;
pub mod r#static;

fn main() {
    let (filepath, ignores) = match parse_args() {
        Some(filepath) => { filepath }
        None => { return; },
    };

    let mut type_env= TypeEnv::new();

    let prelude_path = PathBuf::from(r"prelude.xs");
    let prelude = include_str!(r"./prelude.xs");

    gen_errs_from_src(&prelude_path, prelude, &mut type_env).expect("Prelude can't produce parse errors");

    let mut has_errors = false;
    if let Err(errs) = gen_errs_from_path(&filepath, &mut type_env) {
        has_errors = true;
        for err in errs {
            match err {
                Error::FileErr(msg) => {
                    println!("{}", msg);
                }
                Error::ParseErrs { path, errs } => {
                    print_parse_errs(&path, &errs);
                }
            }
        }
    };

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
