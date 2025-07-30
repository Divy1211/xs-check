use chumsky::prelude::*;
use crate::parsing::lexer::token::Token;
use crate::parsing::span::{Span};

pub fn punctuation<'src>() -> impl Parser<
        'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    one_of("=(){};:,.")
        .to_slice()
        .map(|val| match val {
            "=" => Token::Eq,
            "(" => Token::LParen,
            ")" => Token::RParen,
            "{" => Token::LBrace,
            "}" => Token::RBrace,
            ";" => Token::SColon,
            ":" => Token::Colon,
            "," => Token::Comma,
            _   => Token::Dot,
        })
}
