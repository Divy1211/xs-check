use std::path::PathBuf;

use crate::parsing::ast::{Expr, Literal, Type};
use crate::parsing::span::Spanned;
use crate::r#static::info::{IdInfo, TypeEnv, XSError};
use crate::r#static::type_check::util::{arith_op, logical_op, reln_op, chk_int_lit, chk_num_lit, type_cmp};

pub fn xs_tc_expr(
    path: &PathBuf,
    (expr, span): &Spanned<Expr>,
    type_env: &mut TypeEnv,
) -> Option<Type> { match expr {
    Expr::Literal(lit) => match lit {
        Literal::Int(val) => {
            type_env.add_errs(path, chk_int_lit(&val, &span));
            Some(Type::Int)
        }
        Literal::Float(_) => { Some(Type::Float) }
        Literal::Bool(_) => { Some(Type::Bool) }
        Literal::Str(_) => { Some(Type::Str) }
    }
    Expr::Identifier(id) => {
        let Some(IdInfo { type_, ..}) = type_env.get(id) else {
            type_env.add_err(path, XSError::undefined_name(id, span));
            return None;
        };
        Some(type_)
    }
    Expr::Paren(expr) => { xs_tc_expr(path, expr, type_env) }
    Expr::Vec { x, y, z } => {
        type_env.add_errs(path, chk_num_lit(x, false));
        type_env.add_errs(path, chk_num_lit(y, false));
        type_env.add_errs(path, chk_num_lit(z, false));
        Some(Type::Vec)
    }
    Expr::FnCall { name: (name, name_span), args } => {
        let Some(IdInfo { type_, .. }) = type_env.get(name) else {
            type_env.add_err(path, XSError::undefined_name(name, name_span));
            return None;
        };
        let Type::Func { type_sign, .. } = type_ else {
            type_env.add_err(path, XSError::not_callable(
                name,
                &type_.to_string(),
                name_span,
            ));
            return None;
        };
        for (param_type, arg_expr) in type_sign[..type_sign.len()-1].iter().zip(args) {
            let Some(arg_type) = xs_tc_expr(path, arg_expr, type_env) else {
                // expr will generate its own error if the type cannot be inferred
                continue;
            };
            type_env.add_errs(path, type_cmp(param_type, &arg_type, &arg_expr.1, true, false));
        }
        if args.len() > type_sign.len() {
            for (_expr, span) in args[type_sign.len() - 1..].iter() {
                type_env.add_err(path, XSError::extra_arg(
                    &name.0,
                    span,
                ));
            }
        }

        type_sign.last().map(|val| val.clone())
    }

    Expr::Neg(expr) => {
        let (_, inner_span): &Spanned<Expr> = expr;
        type_env.add_errs(path, chk_num_lit(expr, true));

        if inner_span.start - span.start > 1 {
            type_env.add_err(path, XSError::syntax(
                span,
                "Spaces are not allowed between unary negative ({0}) and {1} literals",
                vec!["-", "int | float"]
            ))
        }

        xs_tc_expr(path, expr, type_env)
    }
    Expr::Not(_) => {
        type_env.add_err(path, XSError::syntax(
            span,
            "Unary not ({0}) is not allowed in XS. yES",
            vec!["!"],
        ));
        Some(Type::Bool)
    }

    Expr::Star(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "multiply")
    }
    Expr::FSlash(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "divide")
    }
    Expr::PCent(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "reduce modulo")
    }

    Expr::Minus(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "subtract")
    }
    Expr::Plus(expr1, expr2) => {
        arith_op(path, span, expr1, expr2, type_env, "add")
    }

    Expr::Lt(expr1, expr2) => {
        reln_op(path, span, expr1, expr2, type_env, "lt")
    }
    Expr::Gt(expr1, expr2) => {
        reln_op(path, span, expr1, expr2, type_env, "gt")
    }
    Expr::Le(expr1, expr2) => {
        reln_op(path, span, expr1, expr2, type_env, "le")
    }
    Expr::Ge(expr1, expr2) => {
        reln_op(path, span, expr1, expr2, type_env, "ge")
    }

    Expr::Eq(expr1, expr2) => {
        reln_op(path, span, expr1, expr2, type_env, "eq")
    }
    Expr::Ne(expr1, expr2) => {
        reln_op(path, span, expr1, expr2, type_env, "ne")
    }

    Expr::And(expr1, expr2) => {
        logical_op(path, span, expr1, expr2, type_env, "and")
    }
    Expr::Or(expr1, expr2) => {
        logical_op(path, span, expr1, expr2, type_env, "or")
    }
}}