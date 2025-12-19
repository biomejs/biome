use crate::cli_options::CliOptions;
use crate::logging::LogOptions;
use crate::runner::CommandRunner;
use crate::{CliDiagnostic, CliSession};

pub(crate) fn run_command(
    session: CliSession,
    log_options: &LogOptions,
    cli_options: &CliOptions,
    mut command: impl CommandRunner,
) -> Result<(), CliDiagnostic> {
    let command = &mut command;
    command.run(session, log_options, cli_options)
}
