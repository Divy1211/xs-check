use chumsky::prelude::*;

use crate::parsing::ast::{ASTreeNode, Body, Expr};
use crate::parsing::lexer::Token;
use crate::parsing::parser::expression::expression;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::body::body;
use crate::parsing::span::{Span, Spanned};

pub fn switch<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<ASTreeNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    let case = just(Token::Case)
        .ignore_then(expression())
        .then_ignore(just(Token::Colon))
        .then(body(statement.clone()))
        .map(|(expr, body)| (Some(expr), body));

    let default = just([Token::Default, Token::Colon])
        .ignore_then(body(statement.clone()))
        .map(|body| (None, body));

    just(Token::Switch)
        .ignore_then(expression().delimited_by(just(Token::LParen), just(Token::RParen)))
        .then(
            choice((case, default))
                .repeated()
                .collect::<Vec<(Option<Spanned<Expr>>, Spanned<Body>)>>()
                .delimited_by(just(Token::LBrace), just(Token::RBrace))
        ).map_with(|(clause, cases), info| (
        ASTreeNode::Switch { clause, cases }, info.span())
    )
}
