use std::collections::HashMap;

use crate::parsing::ast::Identifier;
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::src_loc::SrcLoc;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FnInfo {
    pub identifiers: HashMap<Identifier, IdInfo>,
    pub active_loop_params: HashMap<Identifier, SrcLoc>,
    pub src_loc: SrcLoc
}

impl FnInfo {
    pub fn new(src_loc: SrcLoc) -> Self {
        Self { identifiers: HashMap::new(), active_loop_params: HashMap::new(), src_loc }
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

    pub fn get_active_loop_param(&self, id: &Identifier) -> Option<&SrcLoc> {
        self.active_loop_params.get(id)
    }

    pub fn set_active_loop_param(&mut self, id: Identifier, src_loc: SrcLoc) {
        self.active_loop_params.insert(id, src_loc);
    }

    pub fn unset_active_loop_param(&mut self, id: &Identifier) {
        self.active_loop_params.remove(id);
    }
}
