use crate::parsing::ast::expr::Expr;
use crate::parsing::ast::identifier::Identifier;
use crate::parsing::ast::param::Param;
use crate::parsing::ast::type_::Type;
use crate::parsing::span::Spanned;

pub type Body = Vec<Spanned<ASTreeNode>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RuleOpt {
    Active,
    Inactive,
    RunImmediately,
    HighFrequency,
    MinInterval(Spanned<i64>),
    MaxInterval(Spanned<i64>),
    Priority(Spanned<i64>),
    Group(Spanned<String>),
}

#[derive(Debug, Clone)]
pub enum ASTreeNode {
    Include(Spanned<String>),
    VarDef {
        is_extern: bool,              // no extern inside locals
        is_const: bool,               // only literals can be assigned to consts, no exprs allowed
        is_static: bool,
        type_: Type,
        name: Spanned<Identifier>,
        value: Option<Spanned<Expr>>, // only literals allowed in top level, strings are bugged, vecs are fine. Top levels can't be decls
    },
    VarAssign {
        name: Spanned<Identifier>,
        value: Spanned<Expr>,
    },
    RuleDef {
        name: Spanned<Identifier>,
        rule_opts: Vec<Spanned<RuleOpt>>,
        body: Spanned<Body>,
    },
    FnDef {
        is_mutable: bool,
        return_type: Type,
        name: Spanned<Identifier>,
        params: Vec<Param>,
        body: Spanned<Body>,
    },
    Return (Option<Spanned<Expr>>), // must always be a parenthesized expr
    IfElse {
        condition: Spanned<Expr>,
        consequent: Spanned<Body>,
        alternate: Option<Spanned<Body>>,
    },
    While {
        condition: Spanned<Expr>,
        body: Spanned<Body>,
    },
    For {
        var: Box<Spanned<ASTreeNode>>, // always VarAssign
        condition: Spanned<Expr>,
        body: Spanned<Body>,
    },
    Switch {
        clause: Spanned<Expr>,                              // clause and expr can only be int, float,
        cases: Vec<(Option<Spanned<Expr>>, Spanned<Body>)>, // or bool literals. floats cast to ints
    },
    PostDPlus(Spanned<Identifier>),
    PostDMinus(Spanned<Identifier>),
    Break,
    Continue,
    LabelDef(Spanned<Identifier>),
    Goto(Spanned<Identifier>),
    Discarded(Spanned<Expr>), // only FnCalls are allowed to be discarded.
    
    // How do you use these?
    Debug(Spanned<Identifier>),
    Breakpoint,
    Class {
        name: Spanned<Identifier>,
        member_vars: Vec<Spanned<ASTreeNode>>, // always VarDef, no static/const/externs allowed
    },
}
