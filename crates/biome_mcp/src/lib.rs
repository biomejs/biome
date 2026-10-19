//! A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server
//! that gives coding agents access to Biome.
//!
//! An agent harness starts the server as a child process and exchanges MCP
//! messages with it over standard input and output, one session per process.

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::service::QuitReason;
use rmcp::transport::stdio;
use rmcp::{ServerHandler, ServiceExt, tool_handler, tool_router};
use std::io;

/// Text that clients receive when a session starts. Clients usually pass it to
/// the model to explain what the server is for.
const INSTRUCTIONS: &str = "Biome formats and lints the files of the current project.";

/// The MCP server, which answers the requests of one client.
#[derive(Clone)]
pub struct BiomeServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router(allow_empty)]
impl BiomeServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }
}

impl Default for BiomeServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_handler(router = self.tool_router)]
impl ServerHandler for BiomeServer {
    fn get_info(&self) -> ServerConfig {
        ServerConfig::new(ServerCapabilities::builder().enable_tools().build())
            .with_server_info(Implementation::new("biome", biome_configuration::VERSION))
            .with_instructions(INSTRUCTIONS)
    }
}

/// Serves MCP over standard input and output until the client closes the
/// connection.
///
/// ## Errors
///
/// Returns an error when the session can't start, for example because the
/// client's first message isn't a valid MCP request, or when one of the
/// session's tasks panics or is aborted.
pub async fn serve_stdio() -> io::Result<()> {
    let service = BiomeServer::new()
        .serve(stdio())
        .await
        .map_err(io::Error::other)?;
    match service.waiting().await.map_err(io::Error::other)? {
        QuitReason::JoinError(error) => Err(io::Error::other(error)),
        // The client closing the connection, or the session being cancelled,
        // ends the session normally.
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn describes_itself_and_lists_its_tools() {
        let (server_transport, client_transport) = tokio::io::duplex(4096);
        let server = tokio::spawn(async move {
            let service = BiomeServer::new()
                .serve(server_transport)
                .await
                .expect("server should start");
            service.waiting().await.expect("server should stop cleanly");
        });

        let client = ().serve(client_transport).await.expect("client should connect");
        let info = client.peer_info().expect("server should describe itself");
        let tools = client
            .list_tools(None)
            .await
            .expect("server should list its tools");
        client.cancel().await.expect("client should disconnect");
        server.await.expect("server task should finish");

        let server_info = info
            .server_info
            .as_ref()
            .expect("server should name itself");
        assert_eq!(server_info.name, "biome");
        assert_eq!(server_info.version, biome_configuration::VERSION);
        assert_eq!(info.instructions.as_deref(), Some(INSTRUCTIONS));
        assert!(
            tools.tools.is_empty(),
            "unexpected tools: {:?}",
            tools.tools
        );
    }
}
