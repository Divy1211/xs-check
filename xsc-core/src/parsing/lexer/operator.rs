use chumsky::prelude::*;
use crate::parsing::lexer::token::Token;
use crate::parsing::span::{Span};

pub fn operator<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    one_of("+-*/%<>=!&|")
        .repeated().at_least(1).at_most(2)
        .to_slice()
        .try_map(|val, span| {
            match val {
                "++" => Ok(Token::DPlus),
                "+"  => Ok(Token::Plus),
                "--" => Ok(Token::DMinus),
                "-"  => Ok(Token::Minus),
                "*"  => Ok(Token::Star),
                "/"  => Ok(Token::FSlash),
                "%"  => Ok(Token::FSlash),
                "<=" => Ok(Token::Le),
                "<"  => Ok(Token::Lt),
                ">=" => Ok(Token::Ge),
                ">"  => Ok(Token::Gt),
                "==" => Ok(Token::Deq),
                "!=" => Ok(Token::Neq),
                "&&" => Ok(Token::DAmp),
                "||" => Ok(Token::DPipe),
                "!"  => Ok(Token::Excl),
                _    => Err(Rich::custom(span, "Invalid operator")),
            }
        })
}