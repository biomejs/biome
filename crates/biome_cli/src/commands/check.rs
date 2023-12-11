use crate::cli_options::CliOptions;
use crate::commands::validate_configuration_diagnostics;
use crate::vcs::retrieve_gitignore_matches;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_service::configuration::organize_imports::OrganizeImports;
use biome_service::configuration::{
    load_configuration, FormatterConfiguration, LinterConfiguration, LoadedConfiguration,
};
use biome_service::workspace::{FixFileMode, UpdateSettingsParams};
use biome_service::{Configuration, ConfigurationBasePath, MergeWith};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) struct CheckCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
}

/// Handler for the "check" command of the Biome CLI
pub(crate) fn check(
    mut session: CliSession,
    payload: CheckCommandPayload,
) -> Result<(), CliDiagnostic> {
    let CheckCommandPayload {
        apply,
        apply_unsafe,
        cli_options,
        configuration,
        paths,
        stdin_file_path,
        linter_enabled,
        organize_imports_enabled,
        formatter_enabled,
    } = payload;
    setup_cli_subscriber(cli_options.log_level.clone(), cli_options.log_kind.clone());

    let fix_file_mode = if apply && apply_unsafe {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            "--apply-unsafe",
        ));
    } else if !apply && !apply_unsafe {
        None
    } else if apply && !apply_unsafe {
        Some(FixFileMode::SafeFixes)
    } else {
        Some(FixFileMode::SafeAndUnsafeFixes)
    };

    let base_path = match cli_options.config_path.as_ref() {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(PathBuf::from(path)),
    };

    let loaded_configuration = load_configuration(&session.app.fs, base_path)?;
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
        .get_or_insert_with(FormatterConfiguration::default);

    if formatter_enabled.is_some() {
        formatter.enabled = formatter_enabled;
    }

    let linter = fs_configuration
        .linter
        .get_or_insert_with(LinterConfiguration::default);

    if linter_enabled.is_some() {
        linter.enabled = linter_enabled;
    }

    let organize_imports = fs_configuration
        .organize_imports
        .get_or_insert_with(OrganizeImports::default);

    if organize_imports_enabled.is_some() {
        organize_imports.enabled = organize_imports_enabled;
    }

    fs_configuration.merge_with(configuration);

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        retrieve_gitignore_matches(&session.app.fs, &fs_configuration, vcs_base_path.clone())?;

    let stdin = if let Some(stdin_file_path) = stdin_file_path {
        let console = &mut session.app.console;
        let input_code = console.read();
        if let Some(input_code) = input_code {
            let path = PathBuf::from(stdin_file_path);
            Some((path, input_code))
        } else {
            // we provided the argument without a piped stdin, we bail
            return Err(CliDiagnostic::missing_argument("stdin", "check"));
        }
    } else {
        None
    };

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            configuration: fs_configuration,
            vcs_base_path,
            gitignore_matches,
        })?;

    execute_mode(
        Execution::new(TraversalMode::Check {
            fix_file_mode,
            stdin,
        }),
        session,
        &cli_options,
        paths,
    )
}
