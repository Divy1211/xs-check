use std::collections::HashSet;
use async_trait::async_trait;
use tower_lsp::LanguageServer;
use tower_lsp::lsp_types::{
    DidChangeConfigurationParams,
    DidChangeTextDocumentParams,
    DidCloseTextDocumentParams,
    DidOpenTextDocumentParams,
    InitializeParams,
    InitializeResult,
    InitializedParams,
    SemanticTokens,
    SemanticTokensFullOptions,
    SemanticTokensOptions,
    SemanticTokensParams,
    SemanticTokensResult,
    SemanticTokensServerCapabilities,
    ServerCapabilities,
    ServerInfo,
    TextDocumentSyncCapability,
    TextDocumentSyncKind,
};

use ropey::Rope;

use crate::backend::backend::Backend;
use crate::fmt::pos_info::span_from_pos;
use crate::semantic_tokens::{get_semantic_token_legend, gen_tokens};
use crate::utils::{path_from_uri};

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                semantic_tokens_provider: Some(SemanticTokensServerCapabilities::SemanticTokensOptions(SemanticTokensOptions {
                    legend: get_semantic_token_legend(),
                    full: Some(SemanticTokensFullOptions::Bool(true)),
                    range: None,
                    ..Default::default()
                })),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "XS Check".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.build_prelude_env(false).await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        self.build_prelude_env(false).await;
        let src = Rope::from(params.text_document.text);

        let path = path_from_uri(&uri);
        self.editors.insert(path, (uri.clone(), src));
        self.do_lint(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        let path = path_from_uri(&uri);
        let mut val = self.editors.get_mut(&path).expect("Cached before did_change");

        let (_uri, src) = val.value_mut();
        for change in params.content_changes {
            match change.range {
                None => { 
                    *src = Rope::from(change.text);
                }
                Some(range) => {
                    let span = span_from_pos(&src, range.start, range.end);

                    src.remove(span.start..span.end);
                    src.insert(span.start, &change.text);
                }
            }
        }

        drop(val);
        self.remove_cached(&path);
        self.do_lint(uri).await;
        
        let mut to_relint = HashSet::new();
        for entry in self.dependencies.iter() {
            let (child_path, deps) = entry.pair();
            if !deps.contains(&path) {
                continue;
            }
            let info = self.editors.get(child_path).expect("Dependency cached in do_lint");
            let (uri, _src) = info.value();
            to_relint.insert(uri.clone());
        }
        
        for uri in to_relint {
            self.do_lint(uri).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri;
        let path = path_from_uri(&uri);
        if uri.to_file_path().is_err() {
            self.editors.remove(&path);
            self.dependencies.remove(&path);
            self.remove_cached(&path);
        }
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let path = path_from_uri(&uri);
        let (_uri, src) = &*self.editors.get(&path).expect("Cached before semantic_tokens_full");
        let (_hash, ast) = &*self.ast_cache.get(&path).expect("Cached before semantic_tokens_full");

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: gen_tokens(src, ast),
        })))
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.build_prelude_env(true).await;
        for entry in self.editors.iter() {
            let uri = entry.0.clone();
            self.do_lint(uri).await;
        }
    }
}

