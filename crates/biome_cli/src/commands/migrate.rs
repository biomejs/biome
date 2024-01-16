use crate::cli_options::CliOptions;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{execute_mode, Execution, TraversalMode};
use crate::{setup_cli_subscriber, CliDiagnostic, CliSession};
use biome_service::configuration::{load_configuration, LoadedConfiguration};
use biome_service::ConfigurationBasePath;
use std::path::PathBuf;

/// Handler for the "check" command of the Biome CLI
pub(crate) fn migrate(
    session: CliSession,
    cli_options: CliOptions,
    write: bool,
    prettier: bool,
) -> Result<(), CliDiagnostic> {
    let base_path = match cli_options.config_path.as_ref() {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(PathBuf::from(path)),
    };
    let LoadedConfiguration {
        configuration: _,
        diagnostics: _,
        directory_path,
        file_path,
    } = load_configuration(&session.app.fs, base_path)?;
    setup_cli_subscriber(cli_options.log_level.clone(), cli_options.log_kind.clone());

    if let (Some(path), Some(directory_path)) = (file_path, directory_path) {
        execute_mode(
            Execution::new(TraversalMode::Migrate {
                write,
                configuration_file_path: path,
                configuration_directory_path: directory_path,
                prettier,
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
