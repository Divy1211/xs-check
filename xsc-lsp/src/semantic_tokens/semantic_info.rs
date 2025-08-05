use std::collections::HashMap;
use xsc_core::parsing::ast::Identifier;

pub struct SemanticInfo {
    globals: HashMap<Identifier, (u32, u32)>,
    locals: Option<HashMap<Identifier, (u32, u32)>>,
}

impl SemanticInfo {
    pub fn new() -> SemanticInfo {
        Self {
            globals: HashMap::new(),
            locals: None,
        }
    }
    
    pub fn new_local_env(&mut self) -> Option<HashMap<Identifier, (u32, u32)>> {
        let old = self.locals.take();
        self.locals = Some(HashMap::new());
        old
    }
    
    pub fn set_local_env(&mut self, env: Option<HashMap<Identifier, (u32, u32)>>) {
        self.locals = env;
    }
    
    pub fn get(&self, name: &Identifier) -> Option<&(u32, u32)> {
        self.locals.as_ref()
            .and_then(|env| env.get(name))
            .or_else(|| self.globals.get(name))
    }
    
    pub fn set(&mut self, name: &Identifier, type_: u32, modifiers: u32) {
        match &mut self.locals {
            Some(locals) => { locals.insert(name.clone(), (type_, modifiers)); }
            None => { self.globals.insert(name.clone(), (type_, modifiers)); }
        }
    }
}
