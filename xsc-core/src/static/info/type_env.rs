use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::Arc;
use chumsky::container::{Container};

use crate::parsing::ast::{Identifier};
use crate::parsing::span::{contains, Span};
use crate::r#static::info::fn_info::FnInfo;
use crate::r#static::info::id_info::IdInfo;
use crate::r#static::info::xs_error::XsError;

#[derive(Debug, Clone)]
pub struct TypeEnv {
    pub groups: HashSet<String>,
    pub identifiers: HashMap<Identifier, IdInfo>,
    pub fn_envs: HashMap<Identifier, Vec<FnInfo>>,
    
    pub errs: HashMap<PathBuf, Vec<XsError>>,

    pub current_doc: Option<String>,
    pub current_fnv_env: Option<FnInfo>, // mmm...
    
    pub include_dirs: Arc<Vec<PathBuf>>,
    pub dependencies: Option<HashMap<PathBuf, HashSet<PathBuf>>>,
}

impl TypeEnv {
    
    pub fn errs(&self) -> &HashMap<PathBuf, Vec<XsError>> {
        &self.errs
    }
    
    pub fn new(include_dirs: Vec<PathBuf>) -> Self {
        Self {
            groups: HashSet::new(),
            identifiers: HashMap::new(),
            fn_envs: HashMap::new(),
            errs: HashMap::new(),

            include_dirs: Arc::new(include_dirs),
            dependencies: Some(HashMap::new()),

            current_doc: None,
            current_fnv_env: None,
        }
    }

    pub fn get_mut(&mut self, id: &Identifier) -> Option<&mut IdInfo> {
        self.current_fnv_env.as_mut()
            .and_then(|env| env.get_mut(id))
            .or_else(|| self.identifiers.get_mut(id))
            .map(|val| val)
    }
    
    pub fn get(&self, id: &Identifier) -> Option<IdInfo> {
        self.current_fnv_env.as_ref()
            .and_then(|env| env.get(id))
            .or_else(|| self.identifiers.get(id))
            .map(|val| val.clone())
    }
    
    pub fn set(&mut self, id: &Identifier, info: IdInfo) {
        match &mut self.current_fnv_env {
            Some(env) => env.set(id.clone(), info),
            None => self.identifiers.push((id.clone(), info)),
        }
    }

    pub fn set_global(&mut self, id: &Identifier, info: IdInfo) {
        self.identifiers.push((id.clone(), info))
    }
    
    pub fn get_return(&self) -> Option<IdInfo> {
        self.current_fnv_env.as_ref()
            .and_then(|env| env.get(&Identifier::new("return")))
            .map(|val| val.clone())
    }
    
    pub fn add_group(&mut self, group: &String) {
        self.groups.insert(group.clone());
    }

    pub fn add_err(&mut self, path: &PathBuf, err: XsError) {
        self.errs
            .entry(path.clone())
            .or_insert(vec![])
            .push(err);
    }
    
    pub fn add_errs(&mut self, path: &PathBuf, errs: Vec<XsError>) {
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

    pub fn set_doc(&mut self, doc: String) {
        self.current_doc = Some(doc);
    }

    pub fn take_doc(&mut self) -> Option<String> {
        self.current_doc.take()
    }
    
    pub fn local_ids(&self, path: &PathBuf, span: &Span) -> Option<&HashMap<Identifier, IdInfo>> {
        self.fn_envs
            .values()
            .flatten()
            .filter(|env| {
                let loc = &env.src_loc;
                loc.file_path == *path && contains(&loc.span, span)
            }).map(|env| &env.identifiers).next()
    }
}