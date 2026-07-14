//! Lang.P Language Server — diagnostics, completion, and hover for IDEs.

mod server;

use server::LangpServer;
use tower_lsp::{LspService, Server};

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| LangpServer::new(client));
    Server::new(stdin, stdout, socket).serve(service).await;
}
