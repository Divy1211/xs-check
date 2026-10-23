use std::fmt::Display;
use std::fmt::Formatter;
use std::path::PathBuf;
use crate::parsing::ast::Param;
use crate::parsing::lexer::Token;
use crate::r#static::info::{TypeEnv, WarningKind, XsError};

type TypeSign = Vec<Param>;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Type {
    Int,
    Float,
    Bool,
    Str,
    Vec,
    Void,

    // not real types in XS
    Label,
    Rule,
    Fn { is_mutable: bool, type_sign: TypeSign }, // todo: locals?
    Class,
}

impl Type {
    pub fn signs_are_compatible(new: &TypeSign, prev: &TypeSign, path: &PathBuf, type_env: &mut TypeEnv) -> bool {
        if new.len() != prev.len() {
            return false;
        }
        for (l, r) in new.iter().zip(prev.iter()) {
            if l.name.0 != r.name.0 || l.type_ != r.type_ {
                return false;
            }

            if l.default != r.default {
                type_env.add_err(path, XsError::warning(
                    &l.name.1,
                    "If a {0} function is overridden with different defaults, the defaults of the immediately \
                    preceding definition (whichever is the latest definition seen at the call site) are used to fill \
                    in the missing parameters. However, the function definition that is invoked at runtime is always \
                    the very last definition in the file-chain",
                    vec!["mutable"],
                    WarningKind::InconsistentDefaults,
                ));
            }
        }

        true
    }

    pub fn is_concrete(&self) -> bool {
        matches!(self, Type::Int | Type::Float | Type::Bool | Type::Str | Type::Vec)
    }

    pub fn from_tok(tok: Token) -> Self {
        match tok {
            Token::Int    => Type::Int,
            Token::Bool   => Type::Bool,
            Token::Float  => Type::Float,
            Token::String => Type::Str,
            Token::Vector => Type::Vec,
            Token::Void   => Type::Void,
            _             => panic!("Non type token cannot be converted to type!"),
        }
    }
    
    pub fn accepts(&self, other: &Self) -> bool {
        match (self, other) {
            (left, right) if left == right => true,
            (Type::Int | Type::Float, Type::Int | Type::Float | Type::Bool) => true,
            _ => false,
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "int"),
            Type::Float => write!(f, "float"),
            Type::Bool => write!(f, "bool"),
            Type::Str => write!(f, "string"),
            Type::Vec => write!(f, "vector"),
            Type::Void => write!(f, "void"),

            Type::Label => write!(f, "label"),
            Type::Rule => write!(f, "rule"),
            Type::Fn { is_mutable, type_sign} => write!(
                f,
                "{}{}{}",
                if *is_mutable { "mut " } else { "" },
                if type_sign.len() == 1 { "() -> " } else { "" },
                type_sign
                    .iter()
                    .map(|type_| type_.type_.to_string())
                    .collect::<Vec<String>>()
                    .join(" -> ")
            ),
            Type::Class => write!(f, "class"),
        }
    }
}