use chumsky::prelude::*;

use crate::parsing::ast::ASTreeNode;
use crate::parsing::lexer::Token;
use crate::parsing::parser::parser_input::ParserInput;
use crate::parsing::parser::statement::statement;
use crate::parsing::span::{Span, Spanned};

pub mod expression;
pub mod parser_input;
pub mod statement;

pub fn parser<'tokens>() -> impl Parser<
    'tokens,
    ParserInput<'tokens>,
    Vec<Spanned<ASTreeNode>>,
    extra::Err<Rich<'tokens, Token, Span>>,
> + Clone {
    statement()
        // .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
