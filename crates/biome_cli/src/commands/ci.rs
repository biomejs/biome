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
    pub(crate) configuration: Option<PartialConfiguration>,
    pub(crate) cli_options: CliOptions,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

/// Handler for the "ci" command of the Biome CLI
pub(crate) fn ci(session: CliSession, payload: CiCommandPayload) -> Result<(), CliDiagnostic> {
    let CiCommandPayload {
        cli_options,
        formatter_enabled,
        linter_enabled,
        organize_imports_enabled,
        configuration,
        mut paths,
        since,
        changed,
    } = payload;
    setup_cli_subscriber(cli_options.log_level.clone(), cli_options.log_kind.clone());

    let loaded_configuration =
        load_configuration(&session.app.fs, cli_options.as_configuration_base_path())?;

    validate_configuration_diagnostics(
        &loaded_configuration,
        session.app.console,
        cli_options.verbose,
    )?;

    let LoadedConfiguration {
        configuration: mut fs_configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;
    let formatter = fs_configuration
        .formatter
        .get_or_insert_with(PartialFormatterConfiguration::default);

    if formatter_enabled.is_some() {
        formatter.enabled = formatter_enabled;
    }

    let linter = fs_configuration
        .linter
        .get_or_insert_with(PartialLinterConfiguration::default);

    if linter_enabled.is_some() {
        linter.enabled = linter_enabled;
    }

    let organize_imports = fs_configuration
        .organize_imports
        .get_or_insert_with(PartialOrganizeImports::default);

    if organize_imports_enabled.is_some() {
        organize_imports.enabled = organize_imports_enabled;
    }

    // no point in doing the traversal if all the checks have been disabled
    if fs_configuration.is_formatter_disabled()
        && fs_configuration.is_linter_disabled()
        && fs_configuration.is_organize_imports_disabled()
    {
        return Err(CliDiagnostic::incompatible_end_configuration("Formatter, linter and organize imports are disabled, can't perform the command. This is probably and error."));
    }

    if let Some(mut configuration) = configuration {
        if let Some(linter) = configuration.linter.as_mut() {
            // Don't overwrite rules from the CLI configuration.
            // Otherwise, rules that are disabled in the config file might
            // become re-enabled due to the defaults included in the CLI
            // configuration.
            linter.rules = None;
        }
        fs_configuration.merge_with(configuration);
    }

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        fs_configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

    if since.is_some() && !changed {
        return Err(CliDiagnostic::incompatible_arguments("since", "changed"));
    }

    if changed {
        paths = get_changed_files(&session.app.fs, &fs_configuration, since)?;
    }

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            configuration: fs_configuration,
            working_directory: session.app.fs.working_directory(),
            vcs_base_path,
            gitignore_matches,
        })?;

    execute_mode(Execution::new_ci(), session, &cli_options, paths)
}
