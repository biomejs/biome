use crate::project::Project;
use crate::search::{self, SearchParams};
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{Implementation, ServerCapabilities, ServerConfig};
use rmcp::{ServerHandler, tool, tool_handler, tool_router};
use std::sync::Arc;

/// Text that clients receive when a session starts. Clients usually pass it to
/// the model to explain what the server is for.
pub(crate) const INSTRUCTIONS: &str = "Biome formats and lints the files of the current project. Use the `search` tool to find code by its syntax, such as the calls of a function or the uses of an import, instead of searching text.";

/// The MCP server, which answers the requests of one client.
#[derive(Clone)]
pub struct BiomeServer {
    /// The project that the tools work on, or the reason why it couldn't be
    /// created.
    project: Result<Arc<Project>, String>,
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl BiomeServer {
    /// Creates a server for `project`. When the project couldn't be created,
    /// the server still starts, and its tools return the reason as an error.
    pub fn new(project: Result<Arc<Project>, String>) -> Self {
        Self {
            project,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Finds code by its syntax with a GritQL pattern, in the whole project or in the given paths. Write the code to find between backticks, with `$name` variables for the parts that vary, for example `console.log($message)` or `useEffect($callback, [])` between backticks. Matches are grouped by file: the path, then `line:column: source line` for each match, and the report ends with the number of matches. Set `output` to `files` to list only the files with their number of matches, to `matches` to see the whole code of each match, or to `captures` to see only the code that each `$name` variable captured. Prefer it to a text search when you look for a syntax shape rather than a word."
    )]
    async fn search(&self, params: Parameters<SearchParams>) -> Result<String, String> {
        let project = self.project.clone()?;
        tokio::task::spawn_blocking(move || search::search(&project, params.0))
            .await
            .map_err(|error| error.to_string())?
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

#[cfg(test)]
#[path = "server.tests.rs"]
mod tests;
