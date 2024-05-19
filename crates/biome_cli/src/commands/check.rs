use crate::cli_options::CliOptions;
use crate::commands::{
    get_files_to_process, get_stdin, resolve_manifest, validate_configuration_diagnostics,
};
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_configuration::{
    organize_imports::PartialOrganizeImports, PartialConfiguration, PartialFormatterConfiguration,
    PartialLinterConfiguration,
};
use biome_deserialize::Merge;
use biome_service::configuration::PartialConfigurationExt;
use biome_service::workspace::RegisterProjectFolderParams;
use biome_service::{
    configuration::{load_configuration, LoadedConfiguration},
    workspace::UpdateSettingsParams,
};
use std::ffi::OsString;

use super::{determine_fix_file_mode, FixFileModeOptions};

pub(crate) struct CheckCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) configuration: Option<PartialConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

/// Handler for the "check" command of the Biome CLI
pub(crate) fn check(
    session: CliSession,
    payload: CheckCommandPayload,
) -> Result<(), CliDiagnostic> {
    let CheckCommandPayload {
        apply,
        apply_unsafe,
        write,
        fix,
        unsafe_,
        cli_options,
        configuration,
        mut paths,
        stdin_file_path,
        linter_enabled,
        organize_imports_enabled,
        formatter_enabled,
        since,
        staged,
        changed,
    } = payload;
    setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);

    let fix_file_mode = determine_fix_file_mode(
        FixFileModeOptions {
            apply,
            apply_unsafe,
            write,
            fix,
            unsafe_,
        },
        session.app.console,
    )?;

    let loaded_configuration =
        load_configuration(&session.app.fs, cli_options.as_configuration_path_hint())?;
    validate_configuration_diagnostics(
        &loaded_configuration,
        session.app.console,
        cli_options.verbose,
    )?;
    resolve_manifest(&session)?;

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

    let stdin = get_stdin(stdin_file_path, &mut *session.app.console, "check")?;

    if let Some(_paths) =
        get_files_to_process(since, changed, staged, &session.app.fs, &fs_configuration)?
    {
        paths = _paths;
    }

    session
        .app
        .workspace
        .register_project_folder(RegisterProjectFolderParams {
            path: session.app.fs.working_directory(),
            set_as_current_workspace: true,
        })?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            workspace_directory: session.app.fs.working_directory(),
            configuration: fs_configuration,
            vcs_base_path,
            gitignore_matches,
        })?;

    execute_mode(
        Execution::new(TraversalMode::Check {
            fix_file_mode,
            stdin,
        })
        .set_report(&cli_options),
        session,
        &cli_options,
        paths,
    )
}
