use chumsky::prelude::*;
use crate::parsing::ast::AstNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};

pub fn discarded_expr<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    expression()
        .then_ignore(just(Token::SColon))
        .map_with(|expr, info| {
            (AstNode::Discarded(expr), info.span())
        })
}