use std::collections::{HashMap};
use std::path::PathBuf;

use chumsky::container::Container;
use crate::lint::{gen_errs_from_path};
use crate::parsing::ast::{ASTreeNode, RuleOpt, Expr, Identifier, Literal, Type};
use crate::parsing::span::{Span, Spanned};
use crate::r#static::info::{Error, FnInfo, IdInfo, SrcLoc, TypeEnv, WarningKind, XSError};
use crate::r#static::type_check::expression::xs_tc_expr;
use crate::r#static::type_check::util::{chk_rule_opt, combine_results, type_cmp};

pub fn xs_tc_stmt(
    path: &PathBuf,
    (stmt, span): &Spanned<ASTreeNode>,
    type_env: &mut TypeEnv,
    is_top_level: bool,
    is_breakable: bool,
    is_continuable: bool,
) -> Result<(), Vec<Error>> { match stmt {
    // an include statement is always parsed with a string literal
    ASTreeNode::Include((filename, _span)) => {
        if !is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "An {0} statement is only allowed at the top level",
                vec!["include"],
            ));
            return Ok(());
        }
        let mut inc_path = path.clone();
        inc_path.pop();
        inc_path.push(&filename[1..(filename.len()-1)]);
        gen_errs_from_path(&inc_path, type_env)
    }
    ASTreeNode::VarDef {
        is_extern,
        is_static,
        is_const,
        type_,
        name: spanned_name,
        value
    } => {
        let (name, name_span) = spanned_name;
        match type_env.get(name) {
            Some(IdInfo { src_loc: og_src_loc, ..}) => {
                type_env.add_err(path, XSError::redefined_name(
                    name,
                    name_span,
                    &og_src_loc,
                    None,
                ))
            }
            None => {
                type_env.set(name, IdInfo::from(type_, SrcLoc::from(path, name_span)));
            }
        };

        if !is_top_level && *is_extern {
            type_env.add_err(path, XSError::syntax(
                name_span,
                "Local variables cannot be declared as {0}",
                vec!["extern"],
            ));
        }
        let Some(spanned_expr) = value else {
            if *is_const {
                type_env.add_err(path, XSError::syntax(
                    name_span,
                    "Variable declared as {0} must be initialised with a value",
                    vec!["const"],
                ));
            }
            return Ok(());
        };

        let (expr, expr_span) = spanned_expr;

        if is_top_level || *is_const {
            match expr {
                Expr::Literal(Literal::Str(_)) if is_top_level => {
                    type_env.add_err(path, XSError::warning(
                        expr_span,
                        "Top level initialized {0} do not work correctly. yES",
                        vec!["string"],
                        WarningKind::TopStrInit,
                    ));
                }
                Expr::Literal(_) | Expr::Neg(_) | Expr::Vec { .. } => { }
                _ => {
                    type_env.add_err(path, XSError::syntax(
                        expr_span,
                        "Top level or {0} variable initializers must be literals",
                        vec!["const"],
                    ));
                }
            }
        }

        if *is_static {
            match expr {
                Expr::Literal(_) | Expr::Neg(_) | Expr::Vec { .. } => { }
                _ => {
                    type_env.add_err(path, XSError::syntax(
                        expr_span,
                        "{0} variable initializers must be literals",
                        vec!["static"],
                    ));
                }
            }
        }

        let Some(init_type) = xs_tc_expr(path, spanned_expr, type_env) else {
            return Ok(());
        };

        type_env.add_errs(path, type_cmp(type_, &init_type, expr_span, false, false));
        
        Ok(())

    }
    ASTreeNode::VarAssign {
        name: spanned_name,
        value: spanned_expr
    } => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "Assignments are only allowed in a local scope",
                vec![],
            ));
        }

        let (name, name_span) = spanned_name;

        let Some(IdInfo { type_, .. }) = type_env.get(name) else {
            type_env.add_err(path, XSError::undefined_name(
                name,
                name_span,
            ));
            return Ok(());
        };

        let Some(init_type) = xs_tc_expr(path, spanned_expr, type_env) else {
            // An invalid expr will generate its own error
            return Ok(());
        };

        type_env.add_errs(path, type_cmp(&type_, &init_type, &spanned_expr.1, false, false));
        
        Ok(())
    },
    ASTreeNode::RuleDef {
        name: (name, name_span),
        rule_opts, // todo check for dups, add grp names
        body: (body, body_span)
    } => {
        if !is_top_level {
            type_env.add_err(path, XSError::syntax(
                name_span,
                "A rule definition is only allowed at the top level",
                vec![],
            ));
        }

        let mut opt_spans: HashMap<&str, &Span> = HashMap::with_capacity(rule_opts.len());

        for (opt, opt_span) in rule_opts {
            match opt {
                RuleOpt::Active | RuleOpt::Inactive => {
                    chk_rule_opt("activity", opt_span, &mut opt_spans, path, type_env);
                }
                RuleOpt::RunImmediately => {
                    chk_rule_opt("run immediately", opt_span, &mut opt_spans, path, type_env);
                }
                RuleOpt::HighFrequency => {
                    chk_rule_opt("min interval", opt_span, &mut opt_spans, path, type_env);
                    chk_rule_opt("max interval", opt_span, &mut opt_spans, path, type_env);
                }
                RuleOpt::MinInterval(_) => {
                    chk_rule_opt("min interval", opt_span, &mut opt_spans, path, type_env);
                }
                RuleOpt::MaxInterval(_) => {
                    chk_rule_opt("max interval", opt_span, &mut opt_spans, path, type_env);
                }
                RuleOpt::Priority(_) => {
                    chk_rule_opt("priority", opt_span, &mut opt_spans, path, type_env);
                }
                RuleOpt::Group((grp, _grp_span)) => {
                    if chk_rule_opt("group", opt_span, &mut opt_spans, path, type_env) {
                        type_env.add_group(grp)
                    }
                }
            }
        }

        match type_env.get(name) {
            Some(IdInfo { src_loc: og_src_loc, ..}) => {
                type_env.add_err(path, XSError::redefined_name(
                    name,
                    name_span,
                    &og_src_loc,
                    None,
                ))
            }
            None => {
                type_env.set_global(name, IdInfo::new(Type::Rule, SrcLoc::from(path, name_span)));
            }
        };

        let mut fn_info = FnInfo::new(SrcLoc::from(path, body_span));
        fn_info.set(Identifier::new("return"), IdInfo::dummy(Type::Void));

        let old_env = type_env.get_fn_env();
        type_env.set_fn_env(fn_info);
        // this essentially treats nested fn defs like they're global fns.
        // nested fns aren't allowed in XS so this is fine because we
        // can't close over values
        
        let results = combine_results(body.iter()
            .map(|spanned_stmt| {
                xs_tc_stmt(
                    path, spanned_stmt, type_env,
                    false, is_breakable, is_continuable,
                )
            })
        );
        
        type_env.save_fn_env(name);
        
        // restore the old fn env if it existed
        if let Some(env) = old_env {
            type_env.set_fn_env(env);
        };

        results
    }
    ASTreeNode::FnDef {
        is_mutable,
        return_type,
        name: (name, name_span),
        params,
        body: (body, body_span)
    } => {
        if !is_top_level {
            type_env.add_err(path, XSError::syntax(
                name_span,
                "A function definition is only allowed at the top level",
                vec![],
            ));
        }

        let mut fn_info = FnInfo::new(SrcLoc::from(path, body_span));
        fn_info.set(Identifier::new("return"), IdInfo::dummy(return_type.clone()));
        
        let old_env = type_env.get_fn_env();
        type_env.set_fn_env(fn_info);
        // this essentially treats nested fn defs like they're global fns.
        // nested fns aren't allowed in XS so this is fine because we
        // can't close over values
        
        for param in params {
            let (param_name, param_name_span) = &param.name;
            if let Some(IdInfo {type_: _type, src_loc: og_src_loc}) = type_env.get(param_name) {
                type_env.add_err(path, XSError::redefined_name(
                    name,
                    param_name_span,
                    &og_src_loc,
                    None,
                ))
            }
            
            type_env.set(param_name, IdInfo::from(&param.type_, SrcLoc::from(path, param_name_span)));

            let (expr, expr_span) = &param.default;

            match expr {
                Expr::Literal(_)  | Expr::Neg(_) | Expr::Vec { .. } => { }
                _ => {
                    type_env.add_err(path, XSError::syntax(
                        expr_span,
                        "Parameter defaults must be literals",
                        vec![],
                    ));
                }
            }

            // expr will generate its own error when it returns None
            let Some(param_default_value_type) = xs_tc_expr(path, &param.default, type_env) else {
                continue;
            };
            type_env.add_errs(path, type_cmp(
                &param.type_,
                &param_default_value_type,
                expr_span,
                false,
                false
            ));
        }

        let mut new_type_sign = params
            .iter()
            .map(|param| param.type_.clone())
            .collect::<Vec<Type>>();
        new_type_sign.push(return_type.clone());

        // Nested fns are not allowed. If someone has accidentally defined a nested fn, pretend it
        // exists in the global space, an error was already issued for this above.
        match type_env.get(name) {
            Some(IdInfo{ type_: Type::Func {
                is_mutable: was_mutable,
                type_sign
            }, src_loc: og_src_loc }) => if !was_mutable {
                type_env.add_err(path, XSError::redefined_name(
                    name,
                    name_span,
                    &og_src_loc,
                    Some("Only mutable functions may be overridden"),
                ))
            } else if new_type_sign != *type_sign {
                type_env.add_err(path, XSError::redefined_name(
                    name,
                    name_span,
                    &og_src_loc,
                    Some("Type signatures of mutable functions must be the same"),
                ))
            } else {
                type_env.set_global(name, IdInfo::new(
                     Type::Func { is_mutable: *is_mutable, type_sign: new_type_sign },
                     SrcLoc::from(path, name_span)
                ))
            },
            Some(IdInfo { src_loc: og_src_loc, .. }) => {
                type_env.add_err(path, XSError::redefined_name(
                    name,
                    name_span,
                    &og_src_loc,
                    None,
                ))
            },
            _ => {
                type_env.set_global(name, IdInfo::new(
                    Type::Func { is_mutable: *is_mutable, type_sign: new_type_sign },
                    SrcLoc::from(path, name_span)
                ))
            }
        }

        let results = combine_results(body.iter()
            .map(|spanned_stmt| {
                xs_tc_stmt(
                    path, spanned_stmt, type_env,
                    false, is_breakable, is_continuable,
                )
            })
        );
        
        type_env.save_fn_env(name);

        // restore the old fn env if it existed
        if let Some(env) = old_env {
            type_env.set_fn_env(env);
        };
        
        results
    },
    ASTreeNode::Return(spanned_expr) => {
        let Some(IdInfo { type_: return_type, .. }) = type_env.get_return() else {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed inside functions or rules",
                vec!["return"],
            ));
            return Ok(());
        };

        let Some(spanned_expr) = spanned_expr else {
            if return_type != Type::Void {
                type_env.add_err(path, XSError::type_mismatch(
                    "void",
                    &return_type.to_string(),
                    span,
                    Some(&format!("This function's return type was declared as '{}'", return_type)),
                ));
            }
            return Ok(());
        };
        if return_type == Type::Void {
            type_env.add_err(path, XSError::syntax(
                span,
                "This function's return type was declared as {0}",
                vec!["void"]
            ));
            return Ok(());
        }

        let (expr, expr_span) = spanned_expr;
        if let Expr::Paren(_) = expr {} else {
            type_env.add_err(path, XSError::syntax(
                expr_span,
                "A {0} statement's expression must be enclosed in parenthesis. yES",
                vec!["return"]
            ));
        };

        // if expr returns None, it'll generate its own error
        let Some(return_expr_type) = xs_tc_expr(path, spanned_expr, type_env) else {
            return Ok(());
        };

        type_env.add_errs(path, type_cmp(&return_type, &return_expr_type, expr_span, false, false));

        Ok(())
    },
    ASTreeNode::IfElse {
        condition,
        consequent,
        alternate
    } => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "An {0} statement is only allowed in a local scope",
                vec!["if"]
            ));
        }

        if let Some(type_) = xs_tc_expr(path, condition, type_env) {
            if type_ != Type::Bool {
                type_env.add_err(path, XSError::type_mismatch(
                    &type_.to_string(),
                    "bool",
                    &condition.1,
                    None,
                ));
            }
        }

        let results = consequent.0.iter()
            .map(|spanned_stmt| {
                xs_tc_stmt(
                    path, spanned_stmt, type_env,
                    false, is_breakable, is_continuable,
                )
            })
            .collect::<Vec<_>>();

        if alternate.is_none() {
            return combine_results(results)
        }
        let alternate = alternate.as_ref().expect("Infallible: see above");

        combine_results(results.into_iter().chain(alternate.0.iter()
            .map(|spanned_stmt| {
                xs_tc_stmt(
                    path, spanned_stmt, type_env,
                    false, is_breakable, is_continuable,
                )
            })
        ))
    },
    ASTreeNode::While { condition, body } => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed in a local scope",
                vec!["while"]
            ));
        }

        if let Some(type_) = xs_tc_expr(path, condition, type_env) {
            if type_ != Type::Bool {
                type_env.add_err(path, XSError::type_mismatch(
                    &type_.to_string(),
                    "bool",
                    &condition.1,
                    None,
                ));
            }
        }
        
        combine_results(body.0.iter()
            .map(|spanned_stmt| {
                xs_tc_stmt(
                    path, spanned_stmt, type_env,
                    false, true, true,
                )
            })
        )
    },
    ASTreeNode::For { var, condition, body } => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed in a local scope",
                vec!["for"]
            ));
        }

        let (ASTreeNode::VarAssign { name: (name, name_span), value }, _span) = var.as_ref()
        else { unreachable!("XSC Internal Error while type checking For at {}", var.as_ref().1) };

        /* Redefinitions are allowed for for loop variables */

        // match type_env.get(name) {
        //     Some(IdInfo { src_loc: og_src_loc, .. }) => {
        //         type_env.add_err(path, XSError::redefined_name(
        //             name,
        //             name_span,
        //             &og_src_loc,
        //             None,
        //         ));
        //         return Ok(());
        //     }
        //     _ => {}
        // };

        if let Some(value_type) = xs_tc_expr(path, value, type_env) {
            type_env.add_errs(path, type_cmp(&Type::Int, &value_type, &value.1, false, false));
        }

        type_env.set(name, IdInfo::new(Type::Int, SrcLoc::from(path, name_span)));
        if let Some(type_) = xs_tc_expr(path, condition, type_env) {
            if type_ != Type::Bool {
                type_env.add_err(path, XSError::type_mismatch(
                    &type_.to_string(),
                    "bool",
                    &condition.1,
                    None,
                ));
            }
        }

        combine_results(body.0.iter()
            .map(|spanned_stmt| {
                xs_tc_stmt(
                    path, spanned_stmt, type_env,
                    false, true, true,
                )
            })
        )
    },
    ASTreeNode::Switch { clause, cases } => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed in a local scope",
                vec!["switch"]
            ));
        }

        // expression generates its own error for a None return
        if let Some(clause_type) = xs_tc_expr(path, clause, type_env) {
            type_env.add_errs(path, type_cmp(&Type::Int, &clause_type, &clause.1, false, false));
        }

        let mut default_span: Option<&Span> = None;
        let mut case_spans: HashMap<&Expr, &Span> = HashMap::with_capacity(cases.len());

        let mut results = Vec::with_capacity(cases.len());
        
        for (case_clause, (body, body_span)) in cases {
            // expression generates its own error for a None return
            results.push(combine_results(body.iter()
                .map(|spanned_stmt| {
                    xs_tc_stmt(
                        path, spanned_stmt, type_env,
                        false, true, is_continuable,
                    )
                })
            ));
            let Some(spanned_case_expr) = case_clause else {
                let Some(og_span) = default_span else {
                    default_span = Some(body_span);
                    continue;
                };
                type_env.add_err(path, XSError::warning(
                    og_span,
                    "Only the first default block will run when case matching fails",
                    vec![],
                    WarningKind::DupCase,
                ));
                type_env.add_err(path, XSError::warning(
                    body_span,
                    "Only the first default block will run when case matching fails",
                    vec![],
                    WarningKind::DupCase,
                ));
                continue;
            };
            let (case_expr, case_expr_span) = spanned_case_expr;
            if let Some(clause_type) = xs_tc_expr(path, spanned_case_expr, type_env) {
                type_env.add_errs(path, type_cmp(&Type::Int, &clause_type, case_expr_span, false, true));
            }
            if let Some(&og_span) = case_spans.get(case_expr) {
                type_env.add_err(path, XSError::warning(
                    og_span,
                    "Only the first case will run on a match",
                    vec![],
                    WarningKind::DupCase,
                ));
                type_env.add_err(path, XSError::warning(
                    &spanned_case_expr.1,
                    "Only the first case will run on a match",
                    vec![],
                    WarningKind::DupCase,
                ));
            } else {
                case_spans.push((case_expr, case_expr_span));
            }
        };
        
        combine_results(results)
    },
    ASTreeNode::PostDPlus((id, id_span)) => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A postfix increment ({0}) statement is only allowed in a local scope",
                vec!["++"]
            ));
        }

        let Some(IdInfo { type_: id_type, .. }) = type_env.get(id) else {
            type_env.add_err(path, XSError::undefined_name(id, id_span));
            return Ok(());
        };

        if let Type::Int | Type::Float = id_type {
            return Ok(());
        }
        type_env.add_err(path, XSError::syntax(
            span,
            "A postfix increment ({0}) statement is only allowed on {1} values",
            vec!["++", "int | float"]
        ));
        
        Ok(())
    },
    ASTreeNode::PostDMinus((id, id_span)) => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A postfix decrement ({0}) statement is only allowed in a local scope",
                vec!["--"]
            ));
        }

        let Some(IdInfo { type_: id_type, .. }) = type_env.get(id) else {
            type_env.add_err(path, XSError::undefined_name(id, id_span));
            return Ok(());
        };

        if let Type::Int | Type::Float = id_type {
            return Ok(());
        }
        type_env.add_err(path, XSError::syntax(
            span,
            "A postfix decrement ({0}) statement is only allowed on {1} values",
            vec!["--", "int | float"]
        ));
        
        Ok(())
    },
    ASTreeNode::Break => {
        if !is_breakable {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed inside loops or switch cases",
                vec!["return"],
            ));
        }
        
        Ok(())
    },
    ASTreeNode::Continue => {
        if !is_continuable {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed inside loops",
                vec!["continue"],
            ));
        }

        Ok(())
    },
    ASTreeNode::LabelDef((id, id_span)) => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} definition is only allowed inside a local scope",
                vec!["label"],
            ));
        }

        match type_env.get(id) {
            Some(IdInfo { src_loc: og_src_loc, .. }) => {
                type_env.add_err(path, XSError::redefined_name(
                    id,
                    id_span,
                    &og_src_loc,
                    None,
                ));
                return Ok(());
            }
            _ => {}
        };
        type_env.set(id, IdInfo::new(Type::Label, SrcLoc::from(path, id_span)));

        Ok(())
    },
    ASTreeNode::Goto((id, id_span)) => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed inside functions or rules",
                vec!["goto"],
            ));
        }
        let Some(IdInfo { type_: id_type, .. }) = type_env.get(id) else {
            type_env.add_err(path, XSError::undefined_name(id, id_span));
            return Ok(());
        };

        type_env.add_errs(path, type_cmp(&Type::Label, &id_type, id_span, false, false));

        Ok(())
    },
    ASTreeNode::Discarded(spanned_expr) => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A discarded expression is only allowed in a local scope",
                vec![]
            ));
        }

        let (expr, expr_span) = spanned_expr;
        let Expr::FnCall { .. } = expr else {
            type_env.add_err(path, XSError::syntax(
                expr_span,
                "Only function calls can be discarded",
                vec![],
            ));
            return Ok(());
        };

        // an unknown identifier error will be issued by xs_tc_expr when needed
        let return_value_type = xs_tc_expr(path, spanned_expr, type_env).unwrap_or(Type::Void);

        if let Type::Void = return_value_type {
            return Ok(());
        }
        type_env.add_err(path, XSError::warning(
            expr_span,
            "The return value of this function call is being ignored",
            vec![],
            WarningKind::DiscardedFn,
        ));

        Ok(())
    },
    ASTreeNode::Debug((id, id_span)) => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed inside functions or rules",
                vec!["dbg"],
            ));
        }
        let Some(IdInfo { type_: id_type, .. }) = type_env.get(id) else {
            type_env.add_err(path, XSError::undefined_name(id, id_span));
            return Ok(());
        };

        let (Type::Func { .. } | Type::Rule | Type::Class | Type::Label) = id_type else {
            return Ok(());
        };

        type_env.add_err(path, XSError::syntax(
            id_span,
            "A {0} statement can only be given {1} values",
            vec!["dbg", "int | float | bool | string | vector"],
        ));

        Ok(())
    },
    ASTreeNode::Breakpoint => {
        if is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} statement is only allowed inside a local scope",
                vec!["breakpoint"],
            ));
        }

        type_env.add_err(path, XSError::warning(
            span,
            "Breakpoints cause XS execution to pause irrecoverably",
            vec![],
            WarningKind::BreakPt,
        ));

        Ok(())
    },
    ASTreeNode::Class { name: (id, id_span), member_vars } => {
        if !is_top_level {
            type_env.add_err(path, XSError::syntax(
                span,
                "A {0} definition is only allowed at the top level",
                vec!["class"],
            ));
        }
        if let Some(IdInfo { src_loc: og_src_loc, .. }) = type_env.get(id) {
            type_env.add_err(path, XSError::redefined_name(
                id,
                id_span,
                &og_src_loc,
                None,
            ))
        } else {
            type_env.set(id, IdInfo::new(Type::Class, SrcLoc::from(path, id_span)));
        }

        let mut mem_name: HashMap<&Identifier, &Span> = HashMap::with_capacity(member_vars.len());
        for (member_var, _var_span) in member_vars {
            let ASTreeNode::VarDef {
                type_,
                name: (id, id_span),
                value,
                is_extern,
                is_const,
                is_static
            } = member_var
            else { unreachable!("XSC Internal Error while type checking Class {}", id_span) };

            if *is_extern {
                type_env.add_err(path, XSError::syntax(
                    id_span,
                    "Member variables cannot be declared as {0}",
                    vec!["extern"],
                ));
            }
            if *is_const {
                type_env.add_err(path, XSError::syntax(
                    id_span,
                    "Member variables cannot be declared as {0}",
                    vec!["const"],
                ));
            }
            if *is_static {
                type_env.add_err(path, XSError::syntax(
                    id_span,
                    "Member variables cannot be declared as {0}",
                    vec!["static"],
                ));
            }

            if let Some(&og_span) = mem_name.get(id) {
                type_env.add_err(path, XSError::redefined_name(
                    id,
                    id_span,
                    &SrcLoc::from(path, og_span),
                    None,
                ))
            } else {
                mem_name.push((id, id_span));
            }
            let init_value = match value.as_ref() {
                Some(val) => val,
                None => continue,
            };
            let (_init_value_expr, init_value_span) = init_value;

            let Some(init_value_type) = xs_tc_expr(path, init_value, type_env) else {
                continue;
            };
            type_env.add_errs(path, type_cmp(type_, &init_value_type, init_value_span, false, false));
        }

        type_env.add_err(path, XSError::warning(
            span,
            "Classes are unusable in XS",
            vec![],
            WarningKind::UnusableClasses,
        ));
        Ok(())
    },
}}
