use ropey::Rope;
use tower_lsp::lsp_types::SemanticToken;

use xsc_core::parsing::ast::{AstNode};
use xsc_core::parsing::span::Spanned;

use crate::fmt::pos_info::pos_from_span;
use crate::semantic_tokens::{TokenModifier, TokenType};
use crate::semantic_tokens::semantic_info::SemanticInfo;
use crate::semantic_tokens::xs_token::XsToken;

// todo: put type env inside semantic info, keep track of the current fn env index.
fn xs_toks(
    (node, _span): &Spanned<AstNode>,
    toks: &mut Vec<XsToken>,
    info: &mut SemanticInfo,
) { match node {
    AstNode::Error => {}
    AstNode::Include(_) => {}
    AstNode::VarDef {
        is_const,
        is_static,
        name: (name, span),
        .. 
    } => {
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
    AstNode::VarAssign { name: (name, span), .. } => {
        let Some((type_, modifiers)) = info.get(name) else { return; };
        toks.push(XsToken::from(span, *type_, *modifiers));
    }
    AstNode::RuleDef { body, .. } => {
        let old = info.new_local_env();
        for stmt in body.0.iter() {
            xs_toks(stmt, toks, info);
        }
        info.set_local_env(old);
    }
    AstNode::FnDef { params, body, .. } => {
        let old = info.new_local_env();
        for param in params {
            toks.push(XsToken::from(&param.name.1, TokenType::PARAMETER, TokenModifier::NONE));
            info.set(&param.name.0, TokenType::PARAMETER, TokenModifier::NONE);
        }
        for stmt in body.0.iter() {
            xs_toks(stmt, toks, info);
        }
        info.set_local_env(old);
    }
    AstNode::Return(_) => {}
    AstNode::IfElse { .. } => {}
    AstNode::While { .. } => {}
    AstNode::For { .. } => {}
    AstNode::Switch { .. } => {}
    AstNode::PostDPlus(_) => {}
    AstNode::PostDMinus(_) => {}
    AstNode::Break => {}
    AstNode::Continue => {}
    AstNode::LabelDef(_) => {}
    AstNode::Goto(_) => {}
    AstNode::Discarded(_) => {}
    AstNode::Debug(_) => {}
    AstNode::Breakpoint => {}
    AstNode::Class { .. } => {}
}}

pub fn gen_tokens(src: &Rope, ast: &Vec<Spanned<AstNode>>) -> Vec<SemanticToken> {
    let mut toks = Vec::new();
    let mut info = SemanticInfo::new();
    for node in ast {
        xs_toks(node, &mut toks, &mut info);
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
