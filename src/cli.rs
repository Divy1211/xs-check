use std::collections::HashSet;
use std::path::PathBuf;

use structopt::StructOpt;

use crate::r#static::info::WarningKind;

fn warnings_from_str(ignores: &str) -> Result<HashSet<u32>, &str> {
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

#[derive(Debug, StructOpt)]
#[structopt(name = "xs-check", about = env!("CARGO_PKG_DESCRIPTION"))]
struct Opt {
    #[structopt(parse(from_os_str))]
    filepath: Option<PathBuf>,
    
    #[structopt(short, long, help = "Show binary version & info")]
    version: bool,
    
    #[structopt(
        short,
        long,
        help = "Comma separated list of names of warnings to ignore",
        parse(try_from_str = warnings_from_str)
    )]
    ignores: Option<HashSet<u32>>,

    #[structopt(
        short,
        long,
        help = "Specify an additional prelude file",
        parse(from_os_str)
    )]
    extra_prelude_path: Option<PathBuf>,

    #[structopt(
        short = "I",
        long,
        help = "Additional directories to search for includes. Comma or space delimited",
        parse(from_os_str)
    )]
    include_dirs: Vec<PathBuf>,
}

include!(concat!(env!("OUT_DIR"), "/build_date.rs"));

fn print_info() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");

    println!("{name} v{version}: {description}");
    println!("Author: {authors}");
    println!("Compiled: {BUILD_DATE}");
}

pub fn parse_args() -> Option<(PathBuf, HashSet<u32>, Option<PathBuf>, Vec<PathBuf>)> {
    let opt = Opt::from_args();
    if opt.version {
        print_info();
        return None;
    }
    
    match opt.filepath {
        None => {
            Opt::clap().print_help().unwrap();
            println!();
            None
        }
        Some(filepath) => {
            Some((
                filepath,
                opt.ignores.unwrap_or_else(HashSet::new),
                opt.extra_prelude_path,
                opt.include_dirs
            ))
        }
    }
}