use crate::doxygen::Doc;
use crate::parsing::ast::{Expr, RuleOpt, Type};
use crate::r#static::info::Modifiers;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
pub struct IdInfo {
    pub type_: Type,
    pub src_loc: SrcLoc,
    pub doc: Doc,
    pub init: Option<Expr>,
    pub modifiers: Modifiers,
}

impl IdInfo {
    pub fn from_with_mods(type_: &Type, src_loc: SrcLoc, doc: Doc, modifiers: Modifiers) -> Self {
        Self { type_: type_.clone(), init: None, src_loc, modifiers, doc }
    }
    
    pub fn from(type_: &Type, src_loc: SrcLoc) -> Self {
        Self { type_: type_.clone(), init: None, src_loc, modifiers: Modifiers::var_none(), doc: Doc::None }
    }

    pub fn new(type_: Type, src_loc: SrcLoc, doc: Doc) -> Self {
        Self { type_, init: None, src_loc, modifiers: Modifiers::var_none(), doc }
    }

    pub fn rule(src_loc: SrcLoc, opts: Vec<RuleOpt>, doc: Doc) -> Self {
        Self { type_: Type::Rule, init: None, src_loc, modifiers: Modifiers::rule(opts), doc }
    }
    
    pub fn dummy(type_: Type) -> Self {
        Self { type_, init: None, src_loc: Default::default(), modifiers: Modifiers::var_none(), doc: Doc::None }
    }
}