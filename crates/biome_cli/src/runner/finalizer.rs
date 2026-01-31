use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::runner::execution::Execution;
use biome_console::Console;
use biome_fs::FileSystem;
use biome_service::Workspace;
use biome_service::projects::ProjectKey;
use std::time::Duration;

pub trait Finalizer {
    /// The type accepted by the trait. That's usually the output of the crawler.
    type Input;

    /// Optional hook to run before finalization. Useful for commands that need
    /// to work with the Workspace before finally finalise the command.
    fn before_finalize(
        _project_key: ProjectKey,
        _fs: &dyn FileSystem,
        _workspace: &dyn Workspace,
        _crawler_output: &mut Self::Input,
    ) -> Result<(), CliDiagnostic> {
        Ok(())
    }

    /// Finalize the command. This is where the command needs to work the final input.
    /// This step can be used to print diagnostics to console.
    fn finalize(payload: FinalizePayload<'_, Self::Input>) -> Result<(), CliDiagnostic>;
}

pub(crate) struct FinalizePayload<'a, I> {
    pub(crate) fs: &'a dyn FileSystem,
    pub(crate) workspace: &'a dyn Workspace,
    pub(crate) scan_duration: Option<Duration>,
    pub(crate) console: &'a mut dyn Console,
    pub(crate) cli_options: &'a CliOptions,
    pub(crate) crawler_output: I,
    pub(crate) execution: &'a dyn Execution,
    pub(crate) paths: Vec<String>,
}
