use crate::cli_options::CliOptions;
use crate::commands::{
    get_files_to_process, get_stdin, resolve_manifest, validate_configuration_diagnostics,
};
use crate::diagnostics::DeprecatedArgument;
use crate::execute::VcsTargeted;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_configuration::vcs::PartialVcsConfiguration;
use biome_configuration::{
    PartialCssFormatter, PartialFilesConfiguration, PartialFormatterConfiguration,
    PartialJavascriptFormatter, PartialJsonFormatter,
};
use biome_console::{markup, ConsoleExt};
use biome_deserialize::Merge;
use biome_diagnostics::PrintDiagnostic;
use biome_service::configuration::{
    load_configuration, LoadedConfiguration, PartialConfigurationExt,
};
use biome_service::workspace::{RegisterProjectFolderParams, UpdateSettingsParams};
use std::ffi::OsString;

use super::check_fix_incompatible_arguments;

pub(crate) struct FormatCommandPayload {
    pub(crate) javascript_formatter: Option<PartialJavascriptFormatter>,
    pub(crate) json_formatter: Option<PartialJsonFormatter>,
    pub(crate) css_formatter: Option<PartialCssFormatter>,
    pub(crate) formatter_configuration: Option<PartialFormatterConfiguration>,
    pub(crate) vcs_configuration: Option<PartialVcsConfiguration>,
    pub(crate) files_configuration: Option<PartialFilesConfiguration>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) paths: Vec<OsString>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

/// Handler for the "format" command of the Biome CLI
pub(crate) fn format(
    session: CliSession,
    payload: FormatCommandPayload,
) -> Result<(), CliDiagnostic> {
    let FormatCommandPayload {
        mut javascript_formatter,
        mut formatter_configuration,
        vcs_configuration,
        mut paths,
        cli_options,
        stdin_file_path,
        files_configuration,
        write,
        fix,
        mut json_formatter,
        css_formatter,
        since,
        staged,
        changed,
    } = payload;
    setup_cli_subscriber(cli_options.log_level, cli_options.log_kind);

    check_fix_incompatible_arguments(super::FixFileModeOptions {
        apply: false,
        apply_unsafe: false,
        write,
        fix,
        unsafe_: false,
    })?;

    let loaded_configuration =
        load_configuration(&session.app.fs, cli_options.as_configuration_path_hint())?;
    validate_configuration_diagnostics(
        &loaded_configuration,
        session.app.console,
        cli_options.verbose,
    )?;
    // let fs = &session.app.fs;
    // let (editorconfig, editorconfig_diagnostics) = {
    //     let search_path = loaded_configuration
    //         .directory_path
    //         .clone()
    //         .unwrap_or_else(|| fs.working_directory().unwrap_or_default());
    //     load_editorconfig(fs, search_path)?
    // };
    // for diagnostic in editorconfig_diagnostics {
    //     session.app.console.error(markup! {
    //         {PrintDiagnostic::simple(&diagnostic)}
    //     })
    // }

    resolve_manifest(&session)?;
    let LoadedConfiguration {
        mut configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;
    // let mut configuration = if let Some(mut configuration) = editorconfig {
    //     // this makes biome configuration take precedence over editorconfig configuration
    //     configuration.merge_with(biome_configuration);
    //     configuration
    // } else {
    //     biome_configuration
    // };

    // TODO: remove in biome 2.0
    let console = &mut *session.app.console;
    if let Some(config) = formatter_configuration.as_mut() {
        if let Some(indent_size) = config.indent_size {
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            if config.indent_width.is_none() {
                config.indent_width = Some(indent_size);
            }
        }
    }
    // TODO: remove in biome 2.0
    if let Some(js_formatter) = javascript_formatter.as_mut() {
        if let Some(indent_size) = js_formatter.indent_size {
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--javascript-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--javascript-formatter-indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            if js_formatter.indent_width.is_none() {
                js_formatter.indent_width = Some(indent_size);
            }
        }

        if let Some(trailing_comma) = js_formatter.trailing_comma {
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--trailing-comma"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--trailing-commas"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            if js_formatter.trailing_commas.is_none() {
                js_formatter.trailing_commas = Some(trailing_comma);
            }
        }
    }
    // TODO: remove in biome 2.0
    if let Some(json_formatter) = json_formatter.as_mut() {
        if let Some(indent_size) = json_formatter.indent_size {
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--json-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--json-formatter-indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            if json_formatter.indent_width.is_none() {
                json_formatter.indent_width = Some(indent_size);
            }
        }
    }

    // merge formatter options
    if !configuration
        .formatter
        .as_ref()
        .is_some_and(PartialFormatterConfiguration::is_disabled)
    {
        let formatter = configuration.formatter.get_or_insert_with(Default::default);
        if let Some(formatter_configuration) = formatter_configuration {
            formatter.merge_with(formatter_configuration);
        }

        formatter.enabled = Some(true);
    }
    if css_formatter.is_some() {
        let css = configuration.css.get_or_insert_with(Default::default);
        css.formatter.merge_with(css_formatter);
    }
    if javascript_formatter.is_some() {
        let javascript = configuration
            .javascript
            .get_or_insert_with(Default::default);
        javascript.formatter.merge_with(javascript_formatter);
    }
    if json_formatter.is_some() {
        let json = configuration.json.get_or_insert_with(Default::default);
        json.formatter.merge_with(json_formatter);
    }

    configuration.files.merge_with(files_configuration);
    configuration.vcs.merge_with(vcs_configuration);

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

    if let Some(_paths) =
        get_files_to_process(since, changed, staged, &session.app.fs, &configuration)?
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
            configuration,
            vcs_base_path,
            gitignore_matches,
        })?;

    let stdin = get_stdin(stdin_file_path, console, "format")?;

    let execution = Execution::new(TraversalMode::Format {
        ignore_errors: cli_options.skip_errors,
        write: write || fix,
        stdin,
        vcs_targeted: VcsTargeted { staged, changed },
    })
    .set_report(&cli_options);

    execute_mode(execution, session, &cli_options, paths)
}
