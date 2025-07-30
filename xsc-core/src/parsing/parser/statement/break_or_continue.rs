use chumsky::prelude::*;

use crate::parsing::ast::ASTreeNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::span::{Span, Spanned};

pub fn break_or_continue_or_breakpt<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Spanned<ASTreeNode>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    one_of([Token::Break, Token::Continue, Token::Breakpoint])
        .then_ignore(just(Token::SColon))
        .map_with(|tok, info| (
            match tok {
                Token::Break    => ASTreeNode::Break,
                Token::Continue => ASTreeNode::Continue,
                _               => ASTreeNode::Breakpoint,
            },
            info.span()
        ))
}