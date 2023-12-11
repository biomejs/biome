use crate::cli_options::CliOptions;
use crate::commands::validate_configuration_diagnostics;
use crate::vcs::retrieve_gitignore_matches;
use crate::{execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution};
use biome_service::configuration::organize_imports::OrganizeImports;
use biome_service::configuration::{
    load_configuration, FormatterConfiguration, LinterConfiguration, LoadedConfiguration,
};
use biome_service::workspace::UpdateSettingsParams;
use biome_service::{Configuration, ConfigurationBasePath, MergeWith};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) struct CiCommandPayload {
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) rome_configuration: Configuration,
    pub(crate) cli_options: CliOptions,
}

/// Handler for the "ci" command of the Biome CLI
pub(crate) fn ci(session: CliSession, payload: CiCommandPayload) -> Result<(), CliDiagnostic> {
    setup_cli_subscriber(
        payload.cli_options.log_level.clone(),
        payload.cli_options.log_kind.clone(),
    );

    let base_path = match payload.cli_options.config_path.as_ref() {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(PathBuf::from(path)),
    };

    let loaded_configuration = load_configuration(&session.app.fs, base_path)?;

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
        .get_or_insert_with(FormatterConfiguration::default);

    if payload.formatter_enabled.is_some() {
        formatter.enabled = payload.formatter_enabled;
    }

    let linter = configuration
        .linter
        .get_or_insert_with(LinterConfiguration::default);

    if payload.linter_enabled.is_some() {
        linter.enabled = payload.linter_enabled;
    }

    let organize_imports = configuration
        .organize_imports
        .get_or_insert_with(OrganizeImports::default);

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

    configuration.merge_with(payload.rome_configuration.files);
    configuration.merge_with(payload.rome_configuration.vcs);
    configuration.merge_with_if(
        payload.rome_configuration.formatter,
        !configuration.is_formatter_disabled(),
    );
    configuration.merge_with_if(
        payload.rome_configuration.organize_imports,
        !configuration.is_organize_imports_disabled(),
    );

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        retrieve_gitignore_matches(&session.app.fs, &configuration, vcs_base_path.clone())?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            configuration,
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
