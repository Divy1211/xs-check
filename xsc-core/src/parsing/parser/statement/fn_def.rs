use chumsky::prelude::*;

use crate::parsing::ast::{AstNode, Param, Type};
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::span::{Span, Spanned};

pub fn fn_def<'tokens>(
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
    let arg = one_of([
        Token::Int, Token::Bool, Token::Float, Token::String, Token::Vector
    ]).then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        )
        .then_ignore(just(Token::Eq))
        .then(expression())
        .map(|((type_, name), default)| Param {
            type_: Type::from_tok(type_),
            name,
            default
        });
    
    just(Token::Mutable).or_not()
        .then(one_of([
            Token::Int, Token::Bool, Token::Float, Token::String, Token::Vector, Token::Void]
        )).then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        )
        .then(
            arg
                .separated_by(just(Token::Comma))
                .allow_trailing()
                .collect::<Vec<Param>>()
                .delimited_by(just(Token::LParen), just(Token::RParen))
        ).then(body(statement))
        .map_with(|
            ((((mutable, return_type), name), params), body),
             info
        | (
            AstNode::FnDef {
                is_mutable: mutable.is_some(),
                return_type: Type::from_tok(return_type),
                name,
                params,
                body,
            },
            info.span(),
        ))
}
