use chumsky::prelude::*;
use crate::parsing::ast::AstNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::span::{Span, Spanned};

pub fn if_else<'tokens>(
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
    just(Token::If)
        .ignore_then(expression().delimited_by(just(Token::LParen), just(Token::RParen)))
        .then(body(statement.clone()))
        .then(just(Token::Else).ignore_then(body(statement)).or_not())
        .map_with(|
            ((condition, consequent), alternate),
             info
        | {
            (AstNode::IfElse { condition, consequent, alternate }, info.span())
        })
}
