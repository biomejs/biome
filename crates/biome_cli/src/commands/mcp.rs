use crate::CliDiagnostic;
use tokio::runtime::Runtime;

/// Serves the Model Context Protocol over standard input and output until the
/// client closes the connection. Nothing else may write to standard output while
/// it runs, because that stream carries the protocol messages.
pub(crate) fn mcp() -> Result<(), CliDiagnostic> {
    let runtime = Runtime::new()?;
    runtime.block_on(biome_mcp::serve_stdio())?;
    Ok(())
}
