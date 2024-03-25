use crate::cli_options::CliOptions;
use crate::commands::{get_stdin, resolve_manifest, validate_configuration_diagnostics};
use crate::execute::ReportMode;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_deserialize::Merge;
use biome_service::configuration::vcs::PartialVcsConfiguration;
use biome_service::configuration::{
    load_configuration, LoadedConfiguration, PartialFilesConfiguration,
};
use biome_service::workspace::UpdateSettingsParams;
use std::ffi::OsString;

pub(crate) struct SearchCommandPayload {
    pub(crate) cli_options: CliOptions,
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) pattern: String,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
}

/// Handler for the "search" command of the Biome CLI
pub(crate) fn search(
    session: CliSession,
    payload: SearchCommandPayload,
) -> Result<(), CliDiagnostic> {
    let SearchCommandPayload {
        cli_options,
        files_configuration,
        paths,
        pattern,
        stdin_file_path,
        vcs_configuration,
    } = payload;
    setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);

    let loaded_configuration =
        load_configuration(&session.app.fs, cli_options.as_configuration_base_path())?;
    validate_configuration_diagnostics(
        &loaded_configuration,
        session.app.console,
        cli_options.verbose,
    )?;
    resolve_manifest(&session)?;

    let LoadedConfiguration {
        mut configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;

    configuration.files.merge_with(files_configuration);
    configuration.vcs.merge_with(vcs_configuration);

    // check if support for git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            working_directory: session.app.fs.working_directory(),
            configuration,
            vcs_base_path,
            gitignore_matches,
        })?;

    let console = &mut *session.app.console;
    let stdin = get_stdin(stdin_file_path, console, "search")?;

    let execution = if cli_options.json {
        Execution::with_report(TraversalMode::Search { pattern, stdin }, ReportMode::Json)
    } else {
        Execution::new(TraversalMode::Search { pattern, stdin })
    };

    execute_mode(execution, session, &cli_options, paths)
}
