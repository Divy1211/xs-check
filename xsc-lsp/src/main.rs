// use tokio::net::TcpListener;
use tower_lsp::{LspService, Server};

mod config;
mod fmt;
mod utils;
mod backend;
mod semantic_tokens;

use backend::backend::Backend;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::with_client(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     env_logger::init();
//
//     let (service, socket) = LspService::new(|client| Backend::with_client(client));
//
//     let listener = TcpListener::bind("127.0.0.1:9257").await?;
//     println!("LSP server listening on 127.0.0.1:9257");
//
//     let (stream, _) = listener.accept().await?;
//     let (read, write) = stream.into_split();
//
//     Server::new(read, write, socket).serve(service).await;
//
//     Ok(())
// }