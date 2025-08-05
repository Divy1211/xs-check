use std::sync::{Arc, OnceLock};
use std::path::{Path, PathBuf};
use tower_lsp::Client;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{
    MessageType,
    Url
};
use ropey::Rope;
use dashmap::DashMap;
use xsc_core::r#static::info::{gen_errs_from_src, AstCache, AstMap, TypeEnv};

use crate::config::config::fetch_config;
use crate::config::ext_config::ExtConfig;
use crate::fmt::errs_to_diags::{parse_errs_to_diags, xs_errs_to_diags};
use crate::utils::path_from_uri;

pub type RawSourceInfo = DashMap<PathBuf, (Url, Rope)>;

pub type EnvCache = DashMap<PathBuf, TypeEnv>; 

pub struct Backend {
    client: Client,
    config: Arc<OnceLock<RwLock<ExtConfig>>>,
    prelude_env: Arc<OnceLock<RwLock<TypeEnv>>>,
    pub editors: RawSourceInfo,
    pub ast_cache: AstCache,
    pub env_cache: EnvCache,
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
        }
    }
    
    pub fn remove_cached(&self, path: &Path) {
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

        let Err(errs) = gen_errs_from_src(&path, &src.to_string(), &mut type_env, &self.ast_cache) else {
            let diags = xs_errs_to_diags(&uri, &type_env.errs, &self.editors, &config.ignores);
            self.env_cache.insert(path, type_env);
            self.client.publish_diagnostics(uri, diags, None).await;
            return;
        };

        self.env_cache.insert(path, type_env);
        let diags = parse_errs_to_diags(&uri, &errs, &self.editors);
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

        gen_errs_from_src(&prelude_path, prelude, &mut type_env, &self.ast_cache).expect("Prelude can't produce parse errors");

        // todo: extra prelude

        if self.prelude_env.get().is_none() {
            self.prelude_env.set(RwLock::from(type_env)).expect("Called with true in initialized");
        } else {
            let mut prelude_env = self.prelude_env.get().expect("Initialized").write().await;
            *prelude_env = type_env;
        }
    }
}

