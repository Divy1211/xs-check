use tower_lsp::{LspService, Server};

mod config;
mod fmt;
mod utils;
mod backend;

use backend::backend::Backend;

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::with_client(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
