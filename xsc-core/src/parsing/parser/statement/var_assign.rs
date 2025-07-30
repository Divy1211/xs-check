use chumsky::prelude::*;

use crate::parsing::ast::ASTreeNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};

pub fn var_assign<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    select! { Token::Identifier(id) => id }
        .map_with(|id, info| (id, info.span()))
        .then_ignore(just(Token::Eq))
        .then(expression())
        .then_ignore(just(Token::SColon))
        .map_with(|(name,  value), info| {
            (ASTreeNode::VarAssign {
                name,
                value,
            }, info.span())
        })
}