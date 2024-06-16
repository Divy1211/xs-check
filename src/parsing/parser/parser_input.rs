use chumsky::input::SpannedInput;
use crate::parsing::lexer::token::Token;
use crate::parsing::span::{Span, Spanned};

pub type ParserInput<'tokens> = SpannedInput<Token, Span, &'tokens [Spanned<Token>]>;
