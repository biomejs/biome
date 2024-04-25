use crate::cli_options::CliOptions;
use crate::commands::{
    get_files_to_process, get_stdin, resolve_manifest, validate_configuration_diagnostics,
};
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_configuration::vcs::PartialVcsConfiguration;
use biome_configuration::{
    PartialConfiguration, PartialFilesConfiguration, PartialLinterConfiguration,
};
use biome_deserialize::Merge;
use biome_service::configuration::{
    load_configuration, LoadedConfiguration, PartialConfigurationExt,
};
use biome_service::workspace::{FixFileMode, RegisterProjectFolderParams, UpdateSettingsParams};
use std::ffi::OsString;

pub(crate) struct LintCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) linter_configuration: Option<PartialLinterConfiguration>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

/// Handler for the "lint" command of the Biome CLI
pub(crate) fn lint(session: CliSession, payload: LintCommandPayload) -> Result<(), CliDiagnostic> {
    let LintCommandPayload {
        apply,
        apply_unsafe,
        cli_options,
        mut linter_configuration,
        mut paths,
        stdin_file_path,
        vcs_configuration,
        files_configuration,
        staged,
        changed,
        since,
    } = payload;
    setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);

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
    fs_configuration.merge_with(PartialConfiguration {
        linter: if fs_configuration
            .linter
            .as_ref()
            .is_some_and(PartialLinterConfiguration::is_disabled)
        {
            None
        } else {
            if let Some(linter) = linter_configuration.as_mut() {
                // Don't overwrite rules from the CLI configuration.
                linter.rules = None;
            }
            linter_configuration
        },
        files: files_configuration,
        vcs: vcs_configuration,
        ..Default::default()
    });

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        fs_configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

    if let Some(_paths) =
        get_files_to_process(since, changed, staged, &session.app.fs, &fs_configuration)?
    {
        paths = _paths;
    }

    let stdin = get_stdin(stdin_file_path, &mut *session.app.console, "lint")?;

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
        Execution::new(TraversalMode::Lint {
            fix_file_mode,
            stdin,
        })
        .set_report(&cli_options),
        session,
        &cli_options,
        paths,
    )
}
