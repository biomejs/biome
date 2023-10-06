use crate::cli_options::CliOptions;
use crate::configuration::{load_configuration, LoadedConfiguration};
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{execute_mode, Execution, TraversalMode};
use crate::{setup_cli_subscriber, CliDiagnostic, CliSession};

/// Handler for the "check" command of the Biome CLI
pub(crate) fn migrate(
    mut session: CliSession,
    cli_options: CliOptions,
    write: bool,
) -> Result<(), CliDiagnostic> {
    let LoadedConfiguration {
        configuration: _,
        diagnostics: _,
        directory_path,
        file_path,
    } = load_configuration(&mut session, &cli_options)?;
    setup_cli_subscriber(cli_options.log_level.clone(), cli_options.log_kind.clone());

    if let (Some(path), Some(directory_path)) = (file_path, directory_path) {
        execute_mode(
            Execution::new(TraversalMode::Migrate {
                write,
                configuration_file_path: path,
                configuration_directory_path: directory_path,
            }),
            session,
            &cli_options,
            vec![],
        )
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Biome couldn't find the configuration file".to_string(),
        }))
    }
}
