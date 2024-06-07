use crate::lang::ast::identifier::Identifier;
use crate::lang::ast::literal::Literal;
use crate::lang::span::Spanned;

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(Identifier),
    Paren(Box<Spanned<Expr>>),
    Vec { x: Box<Spanned<Expr>>, y: Box<Spanned<Expr>>, z: Box<Spanned<Expr>> }, // only int/float literals allowed
    FnCall {
        name: Box<Spanned<Expr>>,
        args: Vec<Spanned<Expr>>
    },

    Neg(Box<Spanned<Expr>>),
    Not(Box<Spanned<Expr>>),
    
    Star(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    FSlash(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    PCent(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    
    Plus(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Minus(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    Lt(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Gt(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Le(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Ge(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    Eq(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    Ne(Box<Spanned<Expr>>, Box<Spanned<Expr>>),

    And(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
    
    Or(Box<Spanned<Expr>>, Box<Spanned<Expr>>),
}
