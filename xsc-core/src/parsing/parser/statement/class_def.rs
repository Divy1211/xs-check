use chumsky::prelude::*;
use crate::parsing::ast::ASTreeNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::var_def::var_def;
use crate::parsing::span::{Span, Spanned};

pub fn class_def<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    just(Token::Class)
        .ignore_then(
            select! { Token::Identifier(id) => id }
                .map_with(|id, info| (id, info.span()))
        )
        .then(
            var_def()
                .repeated()
                .collect::<Vec<Spanned<ASTreeNode>>>()
                .delimited_by(just(Token::LBrace), just(Token::RBrace))
        ).then_ignore(just(Token::SColon))
        .map_with(|
            (name, member_vars),
             info
        | (
            ASTreeNode::Class {
                name,
                member_vars,
            },
            info.span(),
        ))
}
