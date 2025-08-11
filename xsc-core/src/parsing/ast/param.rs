use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::{Spanned};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Param {
    pub type_: Type,
    pub name: Spanned<Identifier>,
    pub default: Spanned<Expr>
}
