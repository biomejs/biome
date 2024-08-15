use crate::cli_options::CliOptions;
use crate::commands::{
    get_files_to_process, get_stdin, resolve_manifest, validate_configuration_diagnostics,
};
use crate::execute::VcsTargeted;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_configuration::analyzer::RuleSelector;
use biome_configuration::css::PartialCssLinter;
use biome_configuration::javascript::PartialJavascriptLinter;
use biome_configuration::json::PartialJsonLinter;
use biome_configuration::vcs::PartialVcsConfiguration;
use biome_configuration::{
    PartialConfiguration, PartialFilesConfiguration, PartialGraphqlLinter,
    PartialLinterConfiguration,
};
use biome_deserialize::Merge;
use biome_service::configuration::{
    load_configuration, LoadedConfiguration, PartialConfigurationExt,
};
use biome_service::workspace::{RegisterProjectFolderParams, UpdateSettingsParams};
use std::ffi::OsString;

use super::{determine_fix_file_mode, FixFileModeOptions};

pub(crate) struct LintCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) linter_configuration: Option<PartialLinterConfiguration>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) only: Vec<RuleSelector>,
    pub(crate) skip: Vec<RuleSelector>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
    pub(crate) javascript_linter: Option<PartialJavascriptLinter>,
    pub(crate) json_linter: Option<PartialJsonLinter>,
    pub(crate) css_linter: Option<PartialCssLinter>,
    pub(crate) graphql_linter: Option<PartialGraphqlLinter>,
}

/// Handler for the "lint" command of the Biome CLI
pub(crate) fn lint(session: CliSession, payload: LintCommandPayload) -> Result<(), CliDiagnostic> {
    let LintCommandPayload {
        apply,
        apply_unsafe,
        write,
        fix,
        unsafe_,
        cli_options,
        mut linter_configuration,
        paths,
        only,
        skip,
        stdin_file_path,
        vcs_configuration,
        files_configuration,
        staged,
        changed,
        since,
        javascript_linter,
        css_linter,
        json_linter,
        graphql_linter,
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

    if css_linter.is_some() {
        let css = fs_configuration.css.get_or_insert_with(Default::default);
        css.linter.merge_with(css_linter);
    }

    if graphql_linter.is_some() {
        let graphql = fs_configuration
            .graphql
            .get_or_insert_with(Default::default);
        graphql.linter.merge_with(graphql_linter);
    }
    if javascript_linter.is_some() {
        let javascript = fs_configuration
            .javascript
            .get_or_insert_with(Default::default);
        javascript.linter.merge_with(javascript_linter);
    }
    if json_linter.is_some() {
        let json = fs_configuration.json.get_or_insert_with(Default::default);
        json.linter.merge_with(json_linter);
    }

    let vcs_targeted_paths =
        get_files_to_process(since, changed, staged, &session.app.fs, &fs_configuration)?;

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        fs_configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

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
            only,
            skip,
            vcs_targeted: VcsTargeted { staged, changed },
        })
        .set_report(&cli_options),
        session,
        &cli_options,
        vcs_targeted_paths.unwrap_or(paths),
    )
}
