use crate::doxygen::Doc;
use crate::parsing::ast::Type;
use crate::r#static::info::Modifiers;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
pub struct IdInfo {
    pub type_: Type,
    pub src_loc: SrcLoc,
    pub doc: Option<Doc>,
    pub modifiers: Modifiers,
}

impl IdInfo {
    pub fn from_with_mods(type_: &Type, src_loc: SrcLoc, doc: Option<Doc>, modifiers: Modifiers) -> Self {
        Self { type_: type_.clone(), src_loc, modifiers, doc }
    }

    pub fn new_with_mods(type_: Type, src_loc: SrcLoc, modifiers: Modifiers) -> Self {
        Self { type_, src_loc, modifiers, doc: None }
    }
    
    pub fn from(type_: &Type, src_loc: SrcLoc) -> Self {
        Self { type_: type_.clone(), src_loc, modifiers: Modifiers::var_none(), doc: None }
    }

    pub fn new(type_: Type, src_loc: SrcLoc, doc: Option<Doc>) -> Self {
        Self { type_, src_loc, modifiers: Modifiers::var_none(), doc }
    }
    
    pub fn dummy(type_: Type) -> Self {
        Self { type_, src_loc: Default::default(), modifiers: Modifiers::var_none(), doc: None }
    }
}