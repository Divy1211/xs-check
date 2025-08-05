use chumsky::prelude::*;
use crate::parsing::ast::AstNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::span::{Span, Spanned};

pub fn while_<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<AstNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<AstNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::While)
        .ignore_then(expression().delimited_by(just(Token::LParen), just(Token::RParen)))
        .then(body(statement.clone()))
        .map_with(|
            (condition, body),
            info
        | {
            (AstNode::While { condition, body }, info.span())
        })
}
