use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use chumsky::container::Container;
use crate::parsing::ast::identifier::Identifier;
use crate::r#static::info::fn_info::FnInfo;
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::xs_error::XSError;

pub struct TypeEnv {
    groups: HashSet<String>,
    identifiers: HashMap<Identifier, IdInfo>,
    fn_envs: HashMap<Identifier, Vec<FnInfo>>,
    
    errs: HashMap<PathBuf, Vec<XSError>>,
    
    current_fnv_env: Option<FnInfo>, // mmm...
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            groups: HashSet::new(),
            identifiers: HashMap::new(),
            fn_envs: HashMap::new(),
            errs: HashMap::new(),
            
            current_fnv_env: None,
        }
    }
    
    pub fn get(&self, id: &Identifier) -> Option<IdInfo> {
        match &self.current_fnv_env {
            None =>              self.identifiers.get(id),
            Some(env) => env.get(id),
        }.map(|val| val.clone())
    }
    
    pub fn set(&mut self, id: &Identifier, info: IdInfo) {
        match &mut self.current_fnv_env {
            None =>                 self.identifiers.push((id.clone(), info)),
            Some(env) => env.set(id.clone(), info),
        }
    }
    
    pub fn get_return(&self) -> Option<IdInfo> {
        self.current_fnv_env.as_ref()
            .and_then(|env| env.get(&Identifier::new("return")))
            .map(|val| val.clone())
    }
    
    pub fn add_group(&mut self, group: &String) {
        self.groups.insert(group.clone());
    }

    pub fn add_err(&mut self, path: &PathBuf, err: XSError) {
        self.errs
            .entry(path.clone())
            .or_insert(vec![])
            .push(err);
    }
    
    pub fn add_errs(&mut self, path: &PathBuf, errs: Vec<XSError>) {
        self.errs
            .entry(path.clone())
            .or_insert(vec![])
            .extend(errs);
    }
    
    pub fn set_fn_env(&mut self, fn_info: FnInfo) {
        self.current_fnv_env = Some(fn_info)
    }
    
    pub fn get_fn_env(&mut self) -> Option<FnInfo> {
        self.current_fnv_env.take()
    }
    
    pub fn save_fn_env(&mut self, name: &Identifier) {
        let fn_env = self.current_fnv_env.take().expect("No current fn env - Bugged call");
        self.fn_envs
            .entry(name.clone())
            .or_insert(vec![])
            .push(fn_env);
    }
}