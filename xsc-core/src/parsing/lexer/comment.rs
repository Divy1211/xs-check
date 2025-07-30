use chumsky::error::Rich;
use chumsky::prelude::*;

use crate::parsing::lexer::token::Token;
use crate::parsing::span::Span;

pub fn comment<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    let line_comment = just("//").ignore_then(none_of("\r\n").repeated());
    let block_comment = just("/*")
        .ignore_then(
            any().and_is(just("*/").not()).repeated()
        ).then_ignore(just("*/"));
    
    line_comment.or(block_comment)
        .to_slice()
        .map(|val: &str| Token::Comment(String::from(val)))
}