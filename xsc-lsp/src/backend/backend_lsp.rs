use std::default::Default;
use std::collections::{HashSet};
use std::path::PathBuf;
use async_trait::async_trait;
use tower_lsp::LanguageServer;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionOptions, CompletionParams, CompletionResponse, DidChangeConfigurationParams, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, Documentation, GotoDefinitionParams, GotoDefinitionResponse, Hover, HoverContents, HoverParams, HoverProviderCapability, InitializeParams, InitializeResult, InitializedParams, InlayHint, InlayHintParams, InsertTextFormat, Location, MarkupContent, MarkupKind, OneOf, Range, SemanticTokens, SemanticTokensFullOptions, SemanticTokensOptions, SemanticTokensParams, SemanticTokensResult, SemanticTokensServerCapabilities, ServerCapabilities, ServerInfo, TextDocumentSyncCapability, TextDocumentSyncKind, Url};

use ropey::Rope;

use xsc_core::parsing::ast::{Type};

use crate::backend::backend::Backend;
use crate::fmt::pos_info::{pos_from_span, span_from_pos};
use crate::inlay_hints::gen_inlay_hints;
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
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(
                        "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars()
                            .map(|c| c.to_string())
                            .collect()
                    ),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                inlay_hint_provider: Some(OneOf::Left(true)),
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
                    let span = span_from_pos(src, &range.start, &range.end);

                    src.remove(span.start..span.end);
                    src.insert(span.start, &change.text);
                }
            }
        }

        drop(val);
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
            self.remove_entry(&path);
        }
    }

    async fn goto_definition(&self, params: GotoDefinitionParams) -> tower_lsp::jsonrpc::Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let path = path_from_uri(&uri);

        let (_url, src) = &*self.editors.get(&path).expect("Cached before def");
        let id = self.get_id(src, &pos);
        let span = span_from_pos(src, &pos, &pos);

        let env = &*self.env_cache.get(&path).expect("Cached before def");

        let info = env.identifiers.get(&id)
            .or_else(|| env.local_ids(&path, &span).and_then(|ids| ids.get(&id)));

        let Some(info) = info else {
            return Ok(None);
        };

        let config = self.config
            .get()
            .expect("Initialized")
            .read()
            .await;

        let prelude_path = PathBuf::from(r"prelude.xs");
        if info.src_loc.file_path == prelude_path || info.src_loc.file_path == *config.extra_prelude_path.as_ref().unwrap_or(&prelude_path) {
            return Ok(None);
        }

        let fallback: Option<(Url, Rope)>;
        let guard: Option<dashmap::mapref::one::Ref<'_, PathBuf, (Url, Rope)>>;

        match self.editors.get(&info.src_loc.file_path) {
            Some(ref_) => {
                fallback = None;
                guard = Some(ref_);
            },
            None => {
                let Ok(url) = Url::from_file_path(&info.src_loc.file_path) else {
                    return Ok(None);
                };
                let Ok(src) = std::fs::read_to_string(&info.src_loc.file_path) else {
                    return Ok(None);
                };

                // did_open will cache this
                fallback = Some((url, Rope::from_str(&src)));
                guard = None;
            }
        };
        let (url, src) = guard
            .as_ref()
            .map(|g| g.value())
            .or(fallback.as_ref())
            .expect("one value exists");

        let range = pos_from_span(src, &info.src_loc.span);
        Ok(Some(GotoDefinitionResponse::Scalar(Location::new(url.clone(), Range::new(range.0, range.1)))))
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let pos = params.text_document_position_params.position;
        let path = path_from_uri(&uri);

        let (_url, src) = &*self.editors.get(&path).expect("Cached before hover");
        let id = self.get_id(src, &pos);
        let span = span_from_pos(src, &pos, &pos);

        let env = &*self.env_cache.get(&path).expect("Cached before hover");

        let info = env.identifiers.get(&id)
            .or_else(|| env.local_ids(&path, &span).and_then(|ids| ids.get(&id)));

        let Some(info) = info else {
            return Ok(None);
        };

        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: info.doc.render(&id, info),
            }),
            range: None,
        }))
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri;
        let path = path_from_uri(&uri);
        let (_uri, src) = &*self.editors.get(&path).expect("Cached before semantic_tokens_full");
        let (_hash, (ast, _comms)) = &*self.ast_cache.get(&path).expect("Cached before semantic_tokens_full");

        let env = &*self.env_cache.get(&path).expect("Cached before semantic_tokens_full");

        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: gen_tokens(src, ast, env),
        })))
    }

    async fn inlay_hint(&self, params: InlayHintParams) -> tower_lsp::jsonrpc::Result<Option<Vec<InlayHint>>> {
        let uri = params.text_document.uri;
        let range = params.range;
        let path = path_from_uri(&uri);
        let (_uri, src) = &*self.editors.get(&path).expect("Cached before inlay_hint");
        let (_hash, (ast, _comms)) = &*self.ast_cache.get(&path).expect("Cached before inlay_hint");

        let env = &*self.env_cache.get(&path).expect("Cached before inlay_hint");

        Ok(Some(gen_inlay_hints(src, ast, env, range)))
    }

    async fn completion(&self, params: CompletionParams) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        let uri = params.text_document_position.text_document.uri;
        let pos = params.text_document_position.position;
        let path = path_from_uri(&uri);

        let (_url, src) = &*self.editors.get(&path).expect("Cached before completion");
        let prefix = self.get_id(src, &pos).0;
        let span = span_from_pos(src, &pos, &pos);

        let env = &*self.env_cache.get(&path).expect("Cached before completion");
        
        let ids = env.local_ids(&path, &span)
            .map(|ids| ids.iter())
            .unwrap_or_else(Default::default)
            .chain(env.identifiers.iter())
            .filter(|(id, _info)| id.0.starts_with(&prefix))
            .map(|(id, info)| {
                let (kind, insert_text) = match info.type_ {
                    Type::Int | Type::Float | Type::Bool | Type::Str | Type::Vec => {
                        (CompletionItemKind::VARIABLE, None)
                    }
                    Type::Rule | Type::Fn { .. } => {
                        (CompletionItemKind::FUNCTION, Some(format!("{}($0)", id.0.clone())))
                    }
                    _ => { (CompletionItemKind::TEXT, None) }
                };
                
                CompletionItem {
                    label: id.0.clone(),
                    kind: Some(kind),
                    detail: Some(format!("{}", info.type_)),
                    insert_text,
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: info.doc.render(id, info),
                    })),
                    deprecated: info.doc.deprecation_reason().map(|_reason| true),
                    ..Default::default()
                }
            }).collect();

        Ok(Some(CompletionResponse::Array(ids)))
    }

    async fn did_change_configuration(&self, _: DidChangeConfigurationParams) {
        self.build_prelude_env(true).await;
        for entry in self.editors.iter() {
            let uri = entry.0.clone();
            self.do_lint(uri).await;
        }
    }
}
