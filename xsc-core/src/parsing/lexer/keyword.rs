use chumsky::prelude::*;

use crate::parsing::ast::Identifier;
use crate::parsing::lexer::token::Token;
use crate::parsing::span::Span;

pub fn keyword<'src>() -> impl Parser<
    'src, &'src str, Token, extra::Err<Rich<'src, char, Span>>
> {
    text::ascii::ident().map(|ident| match ident {
        "include"                => Token::Include,
        "switch"                 => Token::Switch,
        "case"                   => Token::Case,
        "while"                  => Token::While,
        "break"                  => Token::Break,
        "default"                => Token::Default,
        "rule"                   => Token::Rule,
        "if"                     => Token::If,
        "then"                   => Token::Then,
        "else"                   => Token::Else,
        "goto"                   => Token::Goto,
        "label"                  => Token::Label,
        "for"                    => Token::For,
        "dbg"                    => Token::Dbg,
        "return"                 => Token::Return,
        "void"                   => Token::Void,
        "const"                  => Token::Const,
        "priority"               => Token::Priority,
        "minInterval"            => Token::MinInterval,
        "maxInterval"            => Token::MaxInterval,
        "highFrequency"          => Token::HighFrequency,
        "active"                 => Token::Active,
        "inactive"               => Token::Inactive,
        "group"                  => Token::Group,
        // "infiniteLoopLimit"      => Token::InfiniteLoopLimit,
        // "infiniteRecursionLimit" => Token::InfiniteRecursionLimit,
        "breakpoint"             => Token::Breakpoint,
        "static"                 => Token::Static,
        "continue"               => Token::Continue,
        "extern"                 => Token::Extern,
        "export"                 => Token::Export,
        "runImmediately"         => Token::RunImmediately,
        "mutable"                => Token::Mutable,
        "class"                  => Token::Class,
        
        _ if ident.eq_ignore_ascii_case("int")     => Token::Int, // yES
        _ if ident.eq_ignore_ascii_case("bool")    => Token::Bool,
        _ if ident.eq_ignore_ascii_case("float")   => Token::Float,
        _ if ident.eq_ignore_ascii_case("string")  => Token::String,
        _ if ident.eq_ignore_ascii_case("vector")  => Token::Vector,
        _                                          => Token::Identifier(Identifier::new(ident))
    })
}
