use crate::cli_options::CliOptions;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{execute_mode, Execution, TraversalMode};
use crate::{setup_cli_subscriber, CliDiagnostic, CliSession};
use biome_console::{markup, ConsoleExt};
use biome_service::configuration::{load_configuration, LoadedConfiguration};
use biome_service::workspace::RegisterProjectFolderParams;

use super::{check_fix_incompatible_arguments, FixFileModeOptions, MigrateSubCommand};

/// Handler for the "migrate" command of the Biome CLI
pub(crate) fn migrate(
    session: CliSession,
    cli_options: CliOptions,
    write: bool,
    fix: bool,
    sub_command: Option<MigrateSubCommand>,
) -> Result<(), CliDiagnostic> {
    let base_path = cli_options.as_configuration_path_hint();
    let LoadedConfiguration {
        configuration: _,
        diagnostics: _,
        directory_path,
        file_path,
    } = load_configuration(&session.app.fs, base_path)?;
    setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);

    check_fix_incompatible_arguments(FixFileModeOptions {
        apply: false,
        apply_unsafe: false,
        write,
        fix,
        unsafe_: false,
    })?;

    session
        .app
        .workspace
        .register_project_folder(RegisterProjectFolderParams {
            path: session.app.fs.working_directory(),
            set_as_current_workspace: true,
        })?;

    if let (Some(path), Some(directory_path)) = (file_path, directory_path) {
        execute_mode(
            Execution::new(TraversalMode::Migrate {
                write: write || fix,
                configuration_file_path: path,
                configuration_directory_path: directory_path,
                sub_command,
            }),
            session,
            &cli_options,
            vec![],
        )
    } else {
        let console = session.app.console;
        console.log(markup! {
            <Info>"If this project has not yet been set up with Biome yet, please follow the "<Hyperlink href="https://biomejs.dev/guides/getting-started/">"Getting Started guide"</Hyperlink>" first."</Info>
        });
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Biome couldn't find the Biome configuration file.".to_string(),
        }))
    }
}
