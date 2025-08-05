use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, Range, Url};

use xsc_core::r#static::info::{Error, XsError};

use crate::fmt::msg_fmt::msg_fmt;
use crate::fmt::pos_info::pos_from_span;
use crate::backend::backend::RawSourceInfo;

pub fn xs_errs_to_diags(
    uri: &Url,
    errs: &HashMap<PathBuf, Vec<XsError>>,
    editors: &RawSourceInfo,
    ignores: &HashSet<u32>
) -> Vec<Diagnostic> {
    let mut diags = Vec::with_capacity(errs.values().map(|v| v.len()).sum());

    for (path, errs) in errs {
        for err in errs {
            if ignores.contains(&err.code()) {
                continue;
            }
            let (err_uri, src) = &*editors.get(path).expect("Called after cache and do_lint");
            if err_uri != uri {
                continue;
            }

            let mut severity = DiagnosticSeverity::ERROR;
            let (kind, msg, span) = match err {
                XsError::ExtraArg { fn_name, span } => {
                    (
                        "ExtraArg",
                        format!("Extra argument provided to function {}", fn_name),
                        span
                    )
                }
                XsError::TypeMismatch { actual, expected, span, note } => {
                    let msg = match note {
                        None => {
                            format!("Expected type {} but found {}", expected, actual)
                        }
                        Some(note) => {
                            format!("Expected type {} but found {}.\nNote: {}", expected, actual, note)
                        }
                    };
                    (
                        "TypeMismatch",
                        msg,
                        span
                    )
                }
                XsError::NotCallable { name, actual, span } => {
                    (
                        "NotCallable",
                        format!("The variable {} is of type {} and not a function", name, actual),
                        span
                    )
                }
                XsError::OpMismatch { op, type1, type2, span, note } => {
                    let msg = match note {
                        None => {
                            format!("Cannot {} types {} and {}", op, type1, type2)
                        }
                        Some(note) => {
                            format!("Cannot {} types {} and {}\nNote: {}", op, type1, type2, note)
                        }
                    };
                    (
                        "OpMismatch",
                        msg,
                        span
                    )
                }
                XsError::UndefinedName { name, span } => {
                    (
                        "UndefinedName",
                        format!("Name {} is not defined", name),
                        span
                    )
                }
                XsError::RedefinedName { name, span, note, og_src_loc: _ } => {
                    let msg = match note {
                        None => {
                            format!("Name {} is already defined", name)
                        }
                        Some(note) => {
                            format!("Name {} is already defined.\nNote: {}", name, note)
                        }
                    };
                    (
                        "RedefinedName",
                        msg,
                        span
                    )
                }
                XsError::UnresolvedInclude { inc_filename, span } => {
                    (
                        "UnresolvedInclude",
                        format!("Failed to resolve included file {}", inc_filename),
                        span
                    )
                }
                XsError::Syntax { span, msg, keywords } => {
                    (
                        "Syntax",
                        msg_fmt(msg, keywords),
                        span
                    )
                }
                XsError::Warning { span, msg, keywords, kind } => {
                    severity = DiagnosticSeverity::WARNING;
                    (
                        kind.as_str(),
                        msg_fmt(msg, keywords),
                        span
                    )
                }
            };

            let (start, end) = pos_from_span(&src, span);

            diags.push(Diagnostic {
                 range: Range {
                     start,
                     end,
                 },
                 severity: Some(severity),
                 code: None,
                 code_description: None,
                 source: Some("xs-check".to_string()),
                 message: format!("{}: {}", kind, msg),
                 related_information: None,
                 tags: None,
                 data: None,
             });
        }
    }

    diags
}

pub fn parse_errs_to_diags(uri: &Url, errs: &Vec<Error>, editors: &RawSourceInfo) -> Vec<Diagnostic> {
    let mut diags = Vec::with_capacity(errs.len());

    for err in errs {
        match err {
            Error::FileErr(_) => { unreachable!("Internal Error Occurred") }
            Error::ParseErrs { path, errs, .. } => {
                let (err_uri, src) = &*editors.get(path).expect("Infallible");
                if err_uri != uri {
                    continue
                }

                for err in errs {
                    let msg = err.msg();
                    let kind = err.kind();
                    let span = err.span();

                    let (start, end) = pos_from_span(&src, span);

                    diags.push(Diagnostic {
                        range: Range {
                            start,
                            end,
                        },
                        severity: Some(DiagnosticSeverity::ERROR),
                        code: None,
                        code_description: None,
                        source: Some("xs-check".to_string()),
                        message: format!("{}: {}", kind, msg),
                        related_information: None,
                        tags: None,
                        data: None,
                    });
                }
            }
        }
    }

    diags
}