mod token;
mod literal;
mod operator;
mod punctuation;
mod keyword;
mod comment;

pub use token::Token;

use chumsky::prelude::*;

use crate::parsing::span::{Span, Spanned};

use comment::comment;
use keyword::keyword;
use literal::literal;
use operator::operator;
use punctuation::punctuation;

pub fn lexer<'src>() -> impl Parser<
    'src, &'src str, Vec<Spanned<Token>>, extra::Err<Rich<'src, char, Span>>
> {
    choice((
        comment(),
        literal(),
        keyword(),
        operator(),
        punctuation(),
    ))
        .map_with(|tok, info| (tok, info.span()))
        .padded()
        // .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
        .padded()
}
