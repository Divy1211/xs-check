use dashmap::{DashMap, DashSet};
use ropey::Rope;
use std::path::{Path, PathBuf};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{
    MessageType,
    Url
};
use tower_lsp::Client;

use xsc_core::r#static::info::{gen_errs_from_src, AstCache, AstMap, TypeEnv};
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
        
        let diags = match gen_errs_from_src(
            &path, &src.to_string(),
            &mut type_env,
            &self.ast_cache,
            &self.editors
        ) {
            Ok(()) => xs_errs_to_diags(&uri, &type_env.errs, &self.editors, &config.ignores),
            Err(errs) => parse_errs_to_diags(&uri, &errs, &self.editors),
        };

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
                self.client.show_message(MessageType::ERROR, format!("Failed to load config: {}", err)).await;
                return;
            }
        }
    }
    
    pub async fn build_prelude_env(&self, refresh: bool) {
        if self.prelude_env.get().is_some() && !refresh {
            return;
        }
        self.load_config(refresh).await;

        let config = self.config
            .get()
            .expect("Infallible")
            .read()
            .await;

        let mut type_env = TypeEnv::new(config.include_dirs.clone());

        let prelude_path = PathBuf::from(r"prelude.xs");
        let prelude = include_str!(r"../../../xsc-core/prelude.xs");

        gen_errs_from_src(&prelude_path, prelude, &mut type_env, &self.ast_cache, &self.editors)
            .expect("Prelude can't produce parse errors");

        // todo: extra prelude

        if self.prelude_env.get().is_none() {
            self.prelude_env.set(RwLock::from(type_env)).expect("Called with true in initialized");
        } else {
            let mut prelude_env = self.prelude_env.get().expect("Initialized").write().await;
            *prelude_env = type_env;
        }
    }
}

