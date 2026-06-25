use ropey::Rope;
use tower_lsp::lsp_types::SemanticToken;

use xsc_core::parsing::ast::{AstNode, Expr, Identifier, RuleOpt};
use xsc_core::parsing::span::Spanned;
use xsc_core::r#static::info::TypeEnv;
use crate::fmt::pos_info::pos_from_span;
use crate::semantic_tokens::{TokenModifier, TokenType};
use crate::semantic_tokens::semantic_info::SemanticInfo;
use crate::semantic_tokens::xs_token::XsToken;

fn xs_toks_expr(
    (expr, span): &Spanned<Expr>,
    toks: &mut Vec<XsToken>,
    info: &mut SemanticInfo,
    env: &TypeEnv,
) { match expr {
    Expr::Literal(_) => {}
    Expr::Identifier(name) => {
        let (type_, modifiers) = match info.get(name) {
            Some((type_, modifiers)) => (*type_, *modifiers),
            None => { match lookup(name, env) {
                Some(v) => v,
                None => { return; }
            }}
        };
        toks.push(XsToken::from(span, type_, modifiers));
    }
    Expr::Paren(inner) => {
        xs_toks_expr(inner, toks, info, env);
    }
    Expr::Vec { x, y, z } => {
        xs_toks_expr(x, toks, info, env);
        xs_toks_expr(y, toks, info, env);
        xs_toks_expr(z, toks, info, env);
    }
    Expr::FnCall { args, .. } => {
        for arg in args {
            xs_toks_expr(arg, toks, info, env);
        }
    }
    Expr::Neg(inner) | Expr::Not(inner) => {
        xs_toks_expr(inner, toks, info, env);
    }
    Expr::Star(expr1, expr2) |
    Expr::FSlash(expr1, expr2) |
    Expr::PCent(expr1, expr2) |
    Expr::Plus(expr1, expr2) |
    Expr::Minus(expr1, expr2) |
    Expr::Lt(expr1, expr2) |
    Expr::Gt(expr1, expr2) |
    Expr::Le(expr1, expr2) |
    Expr::Ge(expr1, expr2) |
    Expr::Eq(expr1, expr2) |
    Expr::Ne(expr1, expr2) |
    Expr::And(expr1, expr2) |
    Expr::Or(expr1, expr2)
    => {
        xs_toks_expr(expr1, toks, info, env);
        xs_toks_expr(expr2, toks, info, env);
    }
}}

// todo: put type env inside semantic info, keep track of the current fn env index.
fn xs_toks(
    (node, _span): &Spanned<AstNode>,
    toks: &mut Vec<XsToken>,
    info: &mut SemanticInfo,
    env: &TypeEnv,
) { match node {
    AstNode::Error => {}
    AstNode::Include(_) => {}
    AstNode::VarDef {
        is_const,
        is_static,
        name: (name, span),
        value,
        ..
    } => {
        if let Some(value) = value {
            xs_toks_expr(value, toks, info, env);
        }
        let mut modifiers = TokenModifier::NONE;
        if *is_const {
            modifiers += TokenModifier::READONLY;
        }
        if *is_static {
            modifiers += TokenModifier::STATIC;
        }
        toks.push(XsToken::from(span, TokenType::VARIABLE, modifiers));
        info.set(name, TokenType::VARIABLE, modifiers);
    }
    AstNode::VarAssign { name: (name, span), value } => {
        xs_toks_expr(value, toks, info, env);
        let (type_, modifiers) = match info.get(name) {
            Some((type_, modifiers)) => (*type_, *modifiers),
            None => { match lookup(name, env) {
                Some(v) => v,
                None => { return; }
            }}
        };
        toks.push(XsToken::from(span, type_, modifiers));
    }
    AstNode::RuleDef { body, rule_opts, .. } => {
        let old = info.new_local_env();
        for stmt in body.0.iter() {
            xs_toks(stmt, toks, info, env);
        }
        for (opt, _span) in rule_opts {
            match opt {
                RuleOpt::MinInterval(expr) |
                RuleOpt::MaxInterval(expr) |
                RuleOpt::Priority(expr) => {
                    xs_toks_expr(expr, toks, info, env);
                }
                _ => {}
            }
        }
        info.set_local_env(old);
    }
    AstNode::FnDef { params, body, .. } => {
        let old = info.new_local_env();
        for param in params {
            toks.push(XsToken::from(&param.name.1, TokenType::PARAMETER, TokenModifier::NONE));
            xs_toks_expr(&param.default, toks, info, env);
            info.set(&param.name.0, TokenType::PARAMETER, TokenModifier::NONE);
        }
        for stmt in body.0.iter() {
            xs_toks(stmt, toks, info, env);
        }
        info.set_local_env(old);
    }
    AstNode::Return(expr) => {
        if let Some(expr) = expr {
            xs_toks_expr(expr, toks, info, env)
        }
    }
    AstNode::IfElse { condition, consequent, alternate } => {
        xs_toks_expr(condition, toks, info, env);
        for stmt in consequent.0.iter() {
            xs_toks(stmt, toks, info, env);
        }
        if let Some(alternate) = alternate {
            for stmt in alternate.0.iter() {
                xs_toks(stmt, toks, info, env);
            }
        }
    }
    AstNode::While { condition, body } => {
        xs_toks_expr(condition, toks, info, env);
        for stmt in body.0.iter() {
            xs_toks(stmt, toks, info, env);
        }
    }
    AstNode::For { var, condition, body } => {
        xs_toks(var, toks, info, env);
        xs_toks_expr(condition, toks, info, env);
        for stmt in body.0.iter() {
            xs_toks(stmt, toks, info, env);
        }
    }
    AstNode::Switch { clause, cases } => {
        xs_toks_expr(clause, toks, info, env);
        for (case, body) in cases {
            if let Some(case) = case {
                xs_toks_expr(case, toks, info, env);
            }
            for stmt in body.0.iter() {
                xs_toks(stmt, toks, info, env);
            }
        }
    }
    AstNode::PostDPlus((name, span)) | AstNode::PostDMinus((name, span)) => {
        let Some((type_, modifiers)) = info.get(name) else { return; };
        toks.push(XsToken::from(span, *type_, *modifiers));
    }
    AstNode::Break => {}
    AstNode::Continue => {}
    AstNode::LabelDef(_) => {}
    AstNode::Goto(_) => {}
    AstNode::Discarded(expr) => {
        xs_toks_expr(expr, toks, info, env);
    }
    AstNode::Debug(_) => {}
    AstNode::Breakpoint => {}
    AstNode::Class { .. } => {}
}}

fn lookup(name: &Identifier, env: &TypeEnv) -> Option<(u32, u32)> {
    let mut modifiers = TokenModifier::NONE;
    let info = env.get(name)?;
    if info.modifiers.is_const() {
        modifiers += TokenModifier::READONLY;
    }
    if info.modifiers.is_static() {
        modifiers += TokenModifier::STATIC;
    }
    Some((TokenType::VARIABLE, modifiers))
}

pub fn gen_tokens(src: &Rope, ast: &Vec<Spanned<AstNode>>, env: &TypeEnv) -> Vec<SemanticToken> {
    let mut toks = Vec::new();
    let mut info = SemanticInfo::new();
    for node in ast {
        xs_toks(node, &mut toks, &mut info, env);
    }
    
    let mut data = Vec::new();
    let mut last_line = 0;
    let mut last_col = 0;

    for tok in toks {
        let (start, end) = pos_from_span(src, &tok.span);
        let delta_line = start.line - last_line;
        let delta_start = if delta_line == 0 {
            start.character - last_col
        } else {
            start.character
        };
        
        data.push(SemanticToken {
            delta_line,
            delta_start,
            length: end.character - start.character,
            token_type: tok.type_,
            token_modifiers_bitset: tok.modifiers,
        });

        last_line = start.line;
        last_col = start.character;
    }
    
    data
}
