use async_trait::async_trait;
use tower_lsp::LanguageServer;
use tower_lsp::lsp_types::{DidChangeConfigurationParams, DidChangeTextDocumentParams, DidOpenTextDocumentParams, InitializeParams, InitializeResult, InitializedParams, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind};
use ropey::Rope;
use crate::backend::backend::Backend;
use crate::fmt::pos_info::span_from_pos;

#[async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
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
        self.editors.insert(uri.to_string(), src);

        self.do_lint(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        
        let mut src = self.editors.get_mut(&uri.to_string()).expect("Infallible");
        
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

        drop(src);
        self.do_lint(uri).await;
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.build_prelude_env(true).await;
    }
}