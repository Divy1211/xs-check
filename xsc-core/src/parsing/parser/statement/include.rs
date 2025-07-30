use chumsky::prelude::*;

use crate::parsing::ast::{ASTreeNode, Literal};
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};

pub fn include<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::Include)
        .ignore_then(
            select! { Token::Literal(Literal::Str(path)) => path }
                .map_with(|path, info| ASTreeNode::Include((path, info.span())))
        ).then_ignore(just(Token::SColon))
        .map_with(|node, info| (node, info.span()))
}