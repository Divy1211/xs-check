use std::fmt::{Display, Formatter};

use crate::parsing::ast::{Identifier, Literal};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Plus,
    Minus,
    Star,
    FSlash,
    PCent,
    DPlus,
    DMinus,
    Lt,
    Gt,
    Le,
    Ge,
    Deq,
    Neq,
    DAmp,
    DPipe,
    Excl,
    
    Eq,
    LBrace,
    RBrace,
    LParen,
    RParen,
    SColon,
    Colon,
    Comma,
    Dot,
    
    Literal(Literal),
    Identifier(Identifier),
    
    Comment(String),

    Vector,
    Include,
    Switch,
    Case,
    While,
    Break,
    Default,
    Rule,
    If,
    Then,
    Else,
    Goto,
    Label,
    For,
    Dbg,
    Return,
    Void,
    Int,
    Bool,
    Float,
    String,
    Const,
    Priority,
    MinInterval,
    MaxInterval,
    HighFrequency,
    Active,
    Inactive,
    Group,
    // InfiniteLoopLimit,
    // InfiniteRecursionLimit,
    Breakpoint,
    Static,
    Continue,
    Extern,
    Export,
    RunImmediately,
    Mutable,
    Class,
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Plus => { write!(f, "+") }
            Token::Minus => { write!(f, "-") }
            Token::Star => { write!(f, "*") }
            Token::FSlash => { write!(f, "/") }
            Token::PCent => { write!(f, "%") }
            Token::DPlus => { write!(f, "++") }
            Token::DMinus => { write!(f, "--") }
            Token::Lt => { write!(f, "<") }
            Token::Gt => { write!(f, ">") }
            Token::Le => { write!(f, "<=") }
            Token::Ge => { write!(f, ">=") }
            Token::Deq => { write!(f, "==") }
            Token::Neq => { write!(f, "!=") }
            Token::DAmp => { write!(f, "&&") }
            Token::DPipe => { write!(f, "||") }
            Token::Excl => { write!(f, "!") }
            
            Token::Eq => { write!(f, "=") }
            Token::LBrace => { write!(f, "{{") }
            Token::RBrace => { write!(f, "}}") }
            Token::LParen => { write!(f, "(") }
            Token::RParen => { write!(f, ")") }
            Token::SColon => { write!(f, ";") }
            Token::Colon => { write!(f, ":") }
            Token::Comma => { write!(f, ",") }
            Token::Dot => { write!(f, ".") }
            
            Token::Literal(lit) => { write!(f, "{}", lit) }
            Token::Identifier(id) => { write!(f, "{}", id) }
            
            Token::Comment(_) => { write!(f, "comment") }
            
            Token::Vector => { write!(f, "vector") }
            Token::Include => { write!(f, "include") }
            Token::Switch => { write!(f, "switch") }
            Token::Case => { write!(f, "case") }
            Token::While => { write!(f, "while") }
            Token::Break => { write!(f, "break") }
            Token::Default => { write!(f, "default") }
            Token::Rule => { write!(f, "rule") }
            Token::If => { write!(f, "if") }
            Token::Then => { write!(f, "then") }
            Token::Else => { write!(f, "else") }
            Token::Goto => { write!(f, "goto") }
            Token::Label => { write!(f, "label") }
            Token::For => { write!(f, "for") }
            Token::Dbg => { write!(f, "dbg") }
            Token::Return => { write!(f, "return") }
            Token::Void => { write!(f, "void") }
            Token::Int => { write!(f, "int") }
            Token::Bool => { write!(f, "bool") }
            Token::Float => { write!(f, "float") }
            Token::String => { write!(f, "string") }
            Token::Const => { write!(f, "const") }
            Token::Priority => { write!(f, "priority") }
            Token::MinInterval => { write!(f, "minInterval") }
            Token::MaxInterval => { write!(f, "maxInterval") }
            Token::HighFrequency => { write!(f, "highFrequency") }
            Token::Active => { write!(f, "active") }
            Token::Inactive => { write!(f, "inactive") }
            Token::Group => { write!(f, "group") }
            // Token::InfiniteLoopLimit => { write!(f, "infiniteLoopLimit") }
            // Token::InfiniteRecursionLimit => { write!(f, "infiniteRecursionLimit") }
            Token::Breakpoint => { write!(f, "breakpoint") }
            Token::Static => { write!(f, "static") }
            Token::Continue => { write!(f, "continue") }
            Token::Extern => { write!(f, "extern") }
            Token::Export => { write!(f, "export") }
            Token::RunImmediately => { write!(f, "runImmediately") }
            Token::Mutable => { write!(f, "mutable") }
            Token::Class => { write!(f, "class") }
        }
    }
}