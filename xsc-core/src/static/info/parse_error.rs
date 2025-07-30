use std::fmt::Display;

use chumsky::error::Rich;

use crate::parsing::span::Span;

#[derive(Debug)]
pub enum ParseError {
    LexerError { msg: String, span: Span },
    ParseError { msg: String, span: Span },
}

impl ParseError {
    pub fn lex_err<T: Display>(err: &Rich<T>) -> Self {
        Self::LexerError { msg: err.to_string(), span: *err.span() }
    }

    pub fn parse_err<T: Display>(err: &Rich<T>) -> Self {
        Self::ParseError { msg: err.to_string(), span: *err.span() }
    }
    
    pub fn msg(&self) -> &str {
        match self {
            ParseError::LexerError { msg, .. } => msg,
            ParseError::ParseError { msg, .. } => msg,
        }
    }
    
    pub fn span(&self) -> &Span {
        match self {
            ParseError::LexerError { span, .. } => span,
            ParseError::ParseError { span, .. } => span,
        }
    }

    pub fn kind(&self) -> &str {
        match self {
            ParseError::LexerError {  .. } => "LexerError",
            ParseError::ParseError {  .. } => "ParseError",
        }
    }
}