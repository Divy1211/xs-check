use std::collections::HashMap;
use std::path::PathBuf;

use chumsky::container::{Container};

use crate::parsing::ast::{Expr, Literal, Type};
use crate::parsing::span::{Span, Spanned};
use crate::r#static::type_check::expression::xs_tc_expr;
use crate::r#static::info::{WarningKind, XSError, TypeEnv};

pub fn combine_results<T>(results: impl IntoIterator<Item = Result<(), Vec<T>>>) -> Result<(), Vec<T>>  {
    let mut num_errs = 0;
    let errs = results.into_iter()
        .filter_map(|result| match result {
            Ok(()) => { None }
            Err(errs) => { num_errs += errs.len(); Some(errs) }
        }).collect::<Vec<_>>();
    
    if num_errs == 0 {
        return Ok(())
    }
    
    Err(errs.into_iter()
        .fold(Vec::with_capacity(num_errs), |mut acc, res| {
            acc.extend(res);
            acc
        })
    )
}


pub fn chk_int_lit(val: &i64, span: &Span) -> Vec<XSError> {
    if *val < -999_999_999 || 999_999_999 < *val {
        vec![XSError::syntax(
            span,
            "{0} literals cannot have more than 9 digits",
            vec!["int"]
        )]
    } else {
        vec![]
    }
}

pub fn chk_num_lit((expr, span): &Spanned<Expr>, is_neg: bool) -> Vec<XSError> {
    match expr {
        Expr::Neg(expr) => if is_neg {
            vec![XSError::syntax(
                span,
                "Unary negative ({0}) is only allowed before {1} literals",
                vec!["-", "int | float"]
            )]
        } else {
            chk_num_lit(expr, true)
        }
        Expr::Literal(lit) => match lit {
            Literal::Int(val) => { chk_int_lit(val, span) }
            Literal::Float(_) => { vec![] }
            Literal::Bool(_) => {
                vec![XSError::type_mismatch(
                    "bool",
                    "int | float",
                    span,
                    None,
                )]
            }
            Literal::Str(_) => {
                vec![XSError::type_mismatch(
                    "string",
                    "int | float",
                    span,
                    None,
                )]
            }
        }
        _ => {
            vec![XSError::syntax(
                span,
                "Only {0} literals are allowed in vector initialization. You may use the {1} function instead",
                vec!["int | float", "xsVectorSet"]
            )]
        }
    }
}

pub fn arith_op(
    path: &PathBuf,
    span: &Span,
    expr1: &Spanned<Expr>,
    expr2: &Spanned<Expr>,
    type_env: &mut TypeEnv,
    op_name: &str
) -> Option<Type> {
    // no error is returned specifically because if None is returned, an error will have
    // been generated already
    let (Some(type1), Some(type2)) = (
        xs_tc_expr(path, expr1, type_env), xs_tc_expr(path, expr2, type_env)
    ) else {
        return None;
    };

    match (type1, type2) {
        (Type::Int, Type::Int) => { Some(Type::Int) }
        (Type::Int, Type::Float) => {
            type_env.add_err(path, XSError::warning(
                span,
                "This expression yields an {0}, not a {1}. The resulting type of an arithmetic operation depends on its first operand. yES",
                vec!["int", "float"],
                WarningKind::FirstOprArith,
            ));
            Some(Type::Int)
        }

        (Type::Float, Type::Int | Type::Float) => { Some(Type::Float) }

        (Type::Str, _) | (_, Type::Str) if op_name == "add" => { Some(Type::Str) }

        (type1, type2) => {
            type_env.add_err(path, XSError::op_mismatch(
                op_name,
                &type1.to_string(),
                &type2.to_string(),
                span,
                None,
            ));
            None
        }
    }
}

pub fn reln_op(
    path: &PathBuf,
    span: &Span,
    expr1: &Spanned<Expr>,
    expr2: &Spanned<Expr>,
    type_env: &mut TypeEnv,
    op_name: &str
) -> Option<Type> {
    // no error is returned specifically because if None is returned, an error will have
    // been generated already
    let (Some(type1), Some(type2)) = (
        xs_tc_expr(path, expr1, type_env), xs_tc_expr(path, expr2, type_env)
    ) else {
        return None;
    };

    match (type1, type2) {
        (Type::Int | Type::Float, Type::Int | Type::Float) => { Some(Type::Bool) }
        (Type::Str, Type::Str) => { Some(Type::Bool) }
        (Type::Vec, Type::Vec) | (Type::Bool, Type::Bool) => {
            if op_name != "eq" && op_name != "ne" {
                type_env.add_err(path, XSError::warning(
                    span,
                    "This comparison will cause a silent XS crash",
                    vec![],
                    WarningKind::CmpSilentCrash,
                ));
            }
            Some(Type::Bool)
        }

        (type1, type2) => {
            type_env.add_err(path, XSError::op_mismatch(
                "compare",
                &type1.to_string(),
                &type2.to_string(),
                span,
                None,
            ));
            None
        }
    }
}

pub fn logical_op(
    path: &PathBuf,
    span: &Span,
    expr1: &Spanned<Expr>,
    expr2: &Spanned<Expr>,
    type_env: &mut TypeEnv,
    op_name: &str
) -> Option<Type> {
    // no error is returned specifically because if None is returned, an error will have
    // been generated already
    let (Some(type1), Some(type2)) = (
        xs_tc_expr(path, expr1, type_env), xs_tc_expr(path, expr2, type_env)
    ) else {
        return None;
    };

    match (type1, type2) {
        (Type::Bool, Type::Bool) => { Some(Type::Bool) }
        (type1, type2) => {
            type_env.add_err(path, XSError::op_mismatch(
                op_name,
                &type1.to_string(),
                &type2.to_string(),
                span,
                None,
            ));
            None
        }
    }
}

pub fn type_cmp(
    expected: &Type,
    actual: &Type,
    actual_span: &Span,
    is_fn_call: bool,
    is_case_expr: bool,
) -> Vec<XSError> {
    let mut errs = vec![];
    match (expected, actual) {
        (_, _) if *expected == *actual => {},
        (Type::Int, Type::Bool) if is_case_expr => {
            errs.push(XSError::warning(
                actual_span,
                "Using booleans in a case's expression will cause a silent XS crash",
                vec![],
                WarningKind::BoolCaseSilentCrash,
            ));
        }
        (Type::Int, Type::Bool) => {} // yES
        (Type::Int, Type::Float) => {
            errs.push(XSError::warning(
                actual_span,
                "Possible loss of precision due to downcast from a {0} to an {1}",
                vec!["float", "int"],
                WarningKind::NumDownCast,
            ));
        }
        (Type::Float, Type::Int | Type::Bool) => if is_fn_call {
            errs.push(XSError::warning(
                actual_span,
                "Intermediate {0} or {1} values do not get promoted to {2} in a \
                function call, floating point operations on this parameter will not work correctly. \
                Consider explicitly assigning this expression to a temporary {3} variable \
                before passing that as a parameter. yES",
                vec!["int", "bool", "float", "float"],
                WarningKind::NoNumPromo,
            ));
        }
        _ => {
            errs.push(XSError::type_mismatch(
                &actual.to_string(),
                &expected.to_string(),
                actual_span,
                None,
            ))
        }
    };
    errs
}

pub fn chk_rule_opt<'src>(
    opt_type: &'static str,
    opt_span: &'src Span,
    opt_spans: &mut HashMap<&'static str, &'src Span>,
    path: &PathBuf,
    type_env: &mut TypeEnv,
) -> bool {
    if let Some(&og_span) = opt_spans.get(opt_type) {
        type_env.add_err(path, XSError::syntax(
                og_span,
                "Cannot set {0} twice",
                vec![opt_type]
        ));
        type_env.add_err(path, XSError::syntax(
                opt_span,
                "Cannot set {0} twice",
                vec![opt_type]
        ));
        false
    } else {
        opt_spans.push((opt_type, opt_span));
        true
    }
}