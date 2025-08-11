use std::collections::HashMap;

use crate::parsing::ast::Identifier;
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FnInfo {
    pub identifiers: HashMap<Identifier, IdInfo>,
    pub src_loc: SrcLoc
}

impl FnInfo {
    pub fn new(src_loc: SrcLoc) -> Self {
        Self { identifiers: HashMap::new(), src_loc }
    }

    pub fn get_mut(&mut self, id: &Identifier) -> Option<&mut IdInfo> {
        self.identifiers.get_mut(id)
    }
    
    pub fn get(&self, id: &Identifier) -> Option<&IdInfo> {
        self.identifiers.get(id)
    }
    
    pub fn set(&mut self, id: Identifier, info: IdInfo) {
        self.identifiers.insert(id, info);
    }
}