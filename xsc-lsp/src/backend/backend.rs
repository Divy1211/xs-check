use std::sync::{Arc, OnceLock};
use std::path::PathBuf;
use std::str::FromStr;

use tower_lsp::Client;
use tokio::sync::RwLock;
use tower_lsp::lsp_types::{
    MessageType,
    Url
};
use ropey::Rope;
use dashmap::DashMap;

use xsc_core::r#static::info::{gen_errs_from_src, TypeEnv};

use crate::config::config::fetch_config;
use crate::config::ext_config::ExtConfig;
use crate::fmt::errs_to_diags::{parse_errs_to_diags, xs_errs_to_diags};

pub type SourceInfo = DashMap<String, Rope>;

pub struct Backend {
    client: Client,
    config: Arc<OnceLock<RwLock<ExtConfig>>>,
    prelude_env: Arc<OnceLock<RwLock<TypeEnv>>>,
    pub editors: SourceInfo,
}

impl Backend {
    pub fn with_client(client: Client) -> Self {
        Self {
            client,
            config: Arc::new(OnceLock::new()),
            prelude_env: Arc::new(OnceLock::new()),
            editors: DashMap::new(),
        }
    }
    
    pub async fn do_lint(&self, uri: Url) {
        let config = self.config
            .get()
            .expect("Infallible")
            .read()
            .await;

        let mut type_env = self.prelude_env
            .get()
            .expect("Infallible")
            .read()
            .await
            .clone();

        let path = PathBuf::from_str(uri.as_str()).expect("Infallible");
        let src = &*self.editors.get(&uri.to_string()).expect("Infallible");

        let Err(errs) = gen_errs_from_src(&path, &src.to_string(), &mut type_env) else {
            let diags = xs_errs_to_diags(&type_env.errs, &self.editors, &config.ignores);
            self.client.publish_diagnostics(uri, diags, None).await;
            return;
        };

        let diags = parse_errs_to_diags(&errs, &self.editors);
        self.client.publish_diagnostics(uri, diags, None).await;
    }
    
    pub async fn load_config(&self, refresh: bool) {
        if self.config.get().is_some() && !refresh {
            return;
        }
        match fetch_config(&self.client).await {
            Ok(config) if self.config.get().is_none() => {
                self.config.set(RwLock::new(config)).expect("This fn runs once");
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

        gen_errs_from_src(&prelude_path, prelude, &mut type_env).expect("Prelude can't produce parse errors");

        // todo: extra prelude

        if self.prelude_env.get().is_none() {
            self.prelude_env.set(RwLock::from(type_env)).expect("Called with true in initialized");
        } else {
            let mut prelude_env = self.prelude_env.get().expect("Initialized").write().await;
            *prelude_env = type_env;
        }
    }
}

