use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::{Span, Spanned};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Param {
    pub type_: Type,
    pub name: Spanned<Identifier>,
    pub default: Spanned<Expr>
}

impl Param {
    pub fn return_(return_type: &Type) -> Self {
        Self {
            name: ("return".into(), Span::new(0, 0)),
            type_: return_type.clone(),
            default: (Expr::Identifier("return".into()), Span::new(0, 0)),
        }
    }
}
