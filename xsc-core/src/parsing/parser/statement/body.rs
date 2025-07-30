use chumsky::prelude::*;

use crate::parsing::ast::{ASTreeNode, Body};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};


pub fn body<'tokens>(
    statement: impl Parser<
        'tokens,
        ParserInput<'tokens>,
        Spanned<ASTreeNode>,
        extra::Err<Rich<'tokens, Token, Span>>,
    > + Clone
) -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<Body>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    let block = statement.clone()
        .repeated()
        .collect::<Vec<Spanned<ASTreeNode>>>()
        .delimited_by(just(Token::LBrace), just(Token::RBrace))
        .map_with(|stmts, info| (
            stmts, info.span()
        ));
    
    let single = statement
        .map_with(|stmt, info| (
            vec![stmt], info.span()
        ));
    
    choice((
        block,
        single,
    ))
}