use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};

use dashmap::{DashMap, DashSet};
use ropey::Rope;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{MessageType, Position, Url};
use tower_lsp::Client;

use xsc_core::parsing::ast::Identifier;
use xsc_core::r#static::info::{gen_errs_from_path, gen_errs_from_src, AstCache, AstMap, TypeEnv};

use crate::config::config::fetch_config;
use crate::config::ext_config::ExtConfig;
use crate::fmt::errs_to_diags::{parse_errs_to_diags, xs_errs_to_diags};
use crate::utils::path_from_uri;

pub type SrcCache = DashMap<PathBuf, (Url, Rope)>;

pub type EnvCache = DashMap<PathBuf, TypeEnv>; 

pub struct Backend {
    client: Client,
    config: Arc<OnceLock<RwLock<ExtConfig>>>,
    prelude_env: Arc<OnceLock<RwLock<TypeEnv>>>,
    pub editors: SrcCache,
    pub ast_cache: AstCache,
    pub env_cache: EnvCache,
    pub dependencies: DashMap<PathBuf, DashSet<PathBuf>>
}

impl Backend {
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            config: Arc::new(OnceLock::new()),
            prelude_env: Arc::new(OnceLock::new()),
            editors: DashMap::new(),
            ast_cache: AstMap::new(),
            env_cache: DashMap::new(),
            dependencies: DashMap::new(),
        }
    }
    
    pub fn remove_entry(&self, path: &Path) {
        self.editors.remove(path);
        self.dependencies.remove(path);
        self.ast_cache.remove(path);
        self.env_cache.remove(path);
    }
    
    pub async fn do_lint(&self, uri: Url) {
        let config = self.config
            .get()
            .expect("Initialized")
            .read()
            .await;

        let mut type_env = self.prelude_env
            .get()
            .expect("Initialized")
            .read()
            .await
            .clone();
        
        let path = path_from_uri(&uri);
        let (_uri, src) = &*self.editors.get(&path).expect("Cached before do_lint");

        let mut parse_errs = match gen_errs_from_src(
            &path, &src.to_string(),
            &mut type_env,
            &self.ast_cache,
            &self.editors
        ) {
            Ok(()) => vec![],
            Err(errs) => parse_errs_to_diags(&uri, &errs, &self.editors),
        };

        let mut diags = xs_errs_to_diags(&uri, &type_env.errs, &self.editors, &config.ignores);
        diags.append(&mut parse_errs);
        
        let deps = type_env.dependencies.take().expect("New type-env created above");
        self.dependencies.insert(path.clone(), deps.into_values().into_iter().flatten().collect());

        self.env_cache.insert(path, type_env);
        self.client.publish_diagnostics(uri, diags, None).await;
    }
    
    pub async fn load_config(&self, refresh: bool) {
        if self.config.get().is_some() && !refresh {
            return;
        }
        match fetch_config(&self.client).await {
            Ok(config) if self.config.get().is_none() => {
                self.config.set(RwLock::new(config)).expect("Only runs once");
            }
            Ok(new_config) => {
                let mut config = self.config.get().expect("Initialized").write().await;
                *config = new_config;
            }
            Err(err) => {
                self.client.show_message(MessageType::ERROR, format!("XSC: Failed to load config: {}", err)).await;
                return;
            }
        }
    }
    
    pub async fn build_prelude_env(&self, refresh: bool) {
        self.load_config(refresh).await;

        let config = self.config
            .get()
            .expect("Config loaded above")
            .read()
            .await;

        if self.prelude_env.get().is_some() && !refresh {
            return;
        }
        
        let mut type_env = TypeEnv::new(config.include_dirs.clone());

        let prelude_path = PathBuf::from(r"prelude.xs");
        let prelude = include_str!(r"../../../xsc-core/prelude.xs");

        gen_errs_from_src(&prelude_path, prelude, &mut type_env, &self.ast_cache, &self.editors)
            .expect("Prelude can't produce parse errors");

        if let Some(extra_prelude_path) = config.extra_prelude_path.as_ref() { 'extra: {
            let path = PathBuf::from(extra_prelude_path);
            if !path.is_file() {
                self.client.show_message(MessageType::ERROR, "XSC: Extra prelude file not found".to_string()).await;
                break 'extra;
            }
            let result = gen_errs_from_path(extra_prelude_path, &mut type_env, &self.ast_cache, &self.editors);
            let err_count = type_env.errs.get(&path).map_or(0, |errs| {
                errs.iter().filter(|err| !err.is_warning()).count()
            });
            if result.is_err() || err_count > 0 {
                self.client.show_message(MessageType::ERROR, "XSC: Errors found in the extra prelude file".to_string()).await;
            }
        }}

        if self.prelude_env.get().is_none() {
            self.prelude_env.set(RwLock::from(type_env)).expect("Called with true in initialized");
        } else {
            let mut prelude_env = self.prelude_env.get().expect("Initialized").write().await;
            *prelude_env = type_env;
        }
    }

    pub fn get_id(&self, src: &Rope, pos: &Position) -> Identifier {
        let line = src.line(pos.line as usize).to_string();
        let char_idx = pos.character as usize;

        let start = line[..char_idx]
            .rfind(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + 1)
            .unwrap_or(0);
        
        let end = line[char_idx..]
            .find(|c: char| !c.is_alphanumeric() && c != '_')
            .map(|i| i + char_idx)
            .unwrap_or(line.len());

        (&line[start..end]).into()
    }
}

