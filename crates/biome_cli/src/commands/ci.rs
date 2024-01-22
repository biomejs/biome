use crate::changed::get_changed_files;
use crate::cli_options::CliOptions;
use crate::commands::validate_configuration_diagnostics;
use crate::{execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution};
use biome_deserialize::Merge;
use biome_service::configuration::organize_imports::PartialOrganizeImports;
use biome_service::configuration::{
    load_configuration, LoadedConfiguration, PartialFormatterConfiguration,
    PartialLinterConfiguration,
};
use biome_service::workspace::UpdateSettingsParams;
use biome_service::PartialConfiguration;
use std::ffi::OsString;

pub(crate) struct CiCommandPayload {
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) configuration: PartialConfiguration,
    pub(crate) cli_options: CliOptions,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

/// Handler for the "ci" command of the Biome CLI
pub(crate) fn ci(session: CliSession, mut payload: CiCommandPayload) -> Result<(), CliDiagnostic> {
    setup_cli_subscriber(
        payload.cli_options.log_level.clone(),
        payload.cli_options.log_kind.clone(),
    );

    let loaded_configuration = load_configuration(
        &session.app.fs,
        payload.cli_options.as_configuration_base_path(),
    )?;

    validate_configuration_diagnostics(
        &loaded_configuration,
        session.app.console,
        payload.cli_options.verbose,
    )?;

    let LoadedConfiguration {
        mut configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;
    let formatter = configuration
        .formatter
        .get_or_insert_with(PartialFormatterConfiguration::default);

    if payload.formatter_enabled.is_some() {
        formatter.enabled = payload.formatter_enabled;
    }

    let linter = configuration
        .linter
        .get_or_insert_with(PartialLinterConfiguration::default);

    if payload.linter_enabled.is_some() {
        linter.enabled = payload.linter_enabled;
    }

    let organize_imports = configuration
        .organize_imports
        .get_or_insert_with(PartialOrganizeImports::default);

    if payload.organize_imports_enabled.is_some() {
        organize_imports.enabled = payload.organize_imports_enabled;
    }

    // no point in doing the traversal if all the checks have been disabled
    if configuration.is_formatter_disabled()
        && configuration.is_linter_disabled()
        && configuration.is_organize_imports_disabled()
    {
        return Err(CliDiagnostic::incompatible_end_configuration("Formatter, linter and organize imports are disabled, can't perform the command. This is probably and error."));
    }

    configuration.merge_with(payload.configuration);

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

    if payload.since.is_some() && !payload.changed {
        return Err(CliDiagnostic::incompatible_arguments("since", "changed"));
    }

    if payload.changed {
        payload.paths = get_changed_files(&session.app.fs, &configuration, payload.since)?;
    }

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            configuration,
            working_directory: session.app.fs.working_directory(),
            vcs_base_path,
            gitignore_matches,
        })?;

    execute_mode(
        Execution::new_ci(),
        session,
        &payload.cli_options,
        payload.paths,
    )
}
