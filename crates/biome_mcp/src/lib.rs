//! A [Model Context Protocol](https://modelcontextprotocol.io) (MCP) server
//! that gives coding agents access to Biome.
//!
//! An agent harness starts the server as a child process and exchanges MCP
//! messages with it over standard input and output, one session per process.
//! Once the session starts, the server loads the Biome configuration of its
//! working directory, then indexes and watches the project's files, so the
//! index stays current while the agent edits files. When the configuration has
//! errors, the tools report them until the configuration is fixed.

mod project;
mod search;
mod server;
#[cfg(test)]
mod server_test_utils;

pub use project::Project;
pub use search::{PatternLanguage, SearchOutput, SearchParams};
pub use server::BiomeServer;

use biome_fs::OsFileSystem;
use biome_service::WatcherOptions;
use camino::Utf8PathBuf;
use rmcp::ServiceExt;
use rmcp::service::QuitReason;
use rmcp::transport::stdio;
use std::io;
use std::sync::Arc;

/// Serves MCP over standard input and output for the project in the current
/// working directory, until the client closes the connection.
///
/// The server starts even when the project's Biome configuration has errors:
/// the tools report the errors, and the project resumes once they're fixed.
/// It also starts when the project can't be created, and the tools report why.
///
/// ## Errors
///
/// Returns an error when the session can't start because the client's first
/// message isn't a valid MCP request, or when one of the session's tasks
/// panics or is aborted.
pub async fn serve_stdio() -> io::Result<()> {
    let project = create_project().map(Arc::new);
    let service = BiomeServer::new(project.clone())
        .serve(stdio())
        .await
        .map_err(io::Error::other)?;
    if let Ok(project) = project {
        load_in_background(project);
    }
    match service.waiting().await.map_err(io::Error::other)? {
        QuitReason::JoinError(error) => Err(io::Error::other(error)),
        // The client closing the connection, or the session being cancelled,
        // ends the session normally.
        _ => Ok(()),
    }
}

/// Creates the project of the current working directory.
///
/// ## Errors
///
/// Returns a message for the agent when the working directory isn't available
/// or its path isn't valid UTF-8, or when the project's files can't be watched.
fn create_project() -> Result<Project, String> {
    let root = std::env::current_dir()
        .map_err(|error| format!("Biome can't read the working directory: {error}"))?;
    let root = Utf8PathBuf::from_path_buf(root).map_err(|root| {
        format!(
            "Biome can't use the working directory `{}`, because its path isn't valid UTF-8.",
            root.display()
        )
    })?;
    Project::new(
        Arc::new(OsFileSystem::new(root.clone())),
        root,
        WatcherOptions::default(),
    )
    .map_err(|error| format!("Biome can't watch the files of the project: {error}"))
}

/// Loads the configuration of `project`, and indexes and watches its files, on
/// a blocking thread. Doing it after the session started keeps a large project
/// from delaying the handshake; tool calls that arrive meanwhile wait for it.
/// Configuration errors are reported by the tools, which also retry loading.
pub(crate) fn load_in_background(project: Arc<Project>) {
    tokio::task::spawn_blocking(move || {
        let _ = project.ensure_loaded();
    });
}
