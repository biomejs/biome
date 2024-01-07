use crate::changed::get_changed_files;
use crate::cli_options::CliOptions;
use crate::commands::{get_stdin, validate_configuration_diagnostics};
use crate::diagnostics::DeprecatedArgument;
use crate::execute::ReportMode;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_console::{markup, ConsoleExt};
use biome_deserialize::{Merge, NoneState};
use biome_diagnostics::PrintDiagnostic;
use biome_service::configuration::css::CssFormatter;
use biome_service::configuration::json::JsonFormatter;
use biome_service::configuration::vcs::VcsConfiguration;
use biome_service::configuration::{
    load_configuration, CssConfiguration, FilesConfiguration, FormatterConfiguration,
    JavascriptConfiguration, JsonConfiguration, LoadedConfiguration,
};
use biome_service::workspace::UpdateSettingsParams;
use biome_service::{JavascriptFormatter, MergeWith};
use biome_service::{Configuration, ConfigurationBasePath, JavascriptFormatter};
use std::ffi::OsString;

pub(crate) struct FormatCommandPayload {
    pub(crate) javascript_formatter: Option<JavascriptFormatter>,
    pub(crate) json_formatter: Option<JsonFormatter>,
    pub(crate) css_formatter: Option<CssFormatter>,
    pub(crate) formatter_configuration: Option<FormatterConfiguration>,
    pub(crate) vcs_configuration: Option<VcsConfiguration>,
    pub(crate) files_configuration: Option<FilesConfiguration>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) write: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) paths: Vec<OsString>,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
}

/// Handler for the "format" command of the Biome CLI
pub(crate) fn format(
    mut session: CliSession,
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
        mut json_formatter,
        mut css_formatter,
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
        mut configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;
    // TODO: remove in biome 2.0
    if let Some(config) = formatter_configuration.as_mut() {
        if let Some(indent_size) = config.indent_size {
            let console = &mut session.app.console;
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            config.indent_width = Some(indent_size);
        }
    }
    // TODO: remove in biome 2.0
    if let Some(js_formatter) = javascript_formatter.as_mut() {
        if let Some(indent_size) = js_formatter.indent_size {
            let console = &mut session.app.console;
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--javascript-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--javascript-formatter-indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            js_formatter.indent_width = Some(indent_size);
        }
    }
    // TODO: remove in biome 2.0
    if let Some(json_formatter) = json_formatter.as_mut() {
        if let Some(indent_size) = json_formatter.indent_size {
            let console = &mut session.app.console;
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--json-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--json-formatter-indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            json_formatter.indent_width = Some(indent_size);
        }
    }
    // TODO: remove in biome 2.0
    if let Some(css_formatter) = css_formatter.as_mut() {
        if let Some(indent_size) = css_formatter.indent_size {
            let console = &mut session.app.console;
            let diagnostic = DeprecatedArgument::new(markup! {
                "The argument "<Emphasis>"--css-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--css-formatter-indent-width"</Emphasis>" instead."
            });
            console.error(markup! {
                {PrintDiagnostic::simple(&diagnostic)}
            });

            css_formatter.indent_width = Some(indent_size);
        }
    }

    configuration.merge_with(Configuration {
        css: Some(CssConfiguration {
            formatter: css_formatter,
            ..NoneState::none()
        }),
        files: files_configuration,
        formatter: if configuration
            .formatter
            .as_ref()
            .is_some_and(FormatterConfiguration::is_disabled)
        {
            None
        } else {
            Some(FormatterConfiguration {
                enabled: Some(true),
                ..formatter_configuration.unwrap_or_else(NoneState::none)
            })
        },
        javascript: Some(JavascriptConfiguration {
            formatter: javascript_formatter,
            ..NoneState::none()
        }),
        json: Some(JsonConfiguration {
            formatter: json_formatter,
            ..NoneState::none()
        }),
        vcs: vcs_configuration,
        ..NoneState::none()
    });

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    let (vcs_base_path, gitignore_matches) =
        configuration.retrieve_gitignore_matches(&session.app.fs, vcs_base_path.as_deref())?;

    if since.is_some() && !changed {
        return Err(CliDiagnostic::incompatible_arguments("since", "changed"));
    }

    if changed {
        paths = get_changed_files(&session.app.fs, &configuration, since)?;
    }

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            working_directory: session.app.fs.working_directory(),
            configuration,
            vcs_base_path,
            gitignore_matches,
        })?;

    let stdin = get_stdin(stdin_file_path, &mut *session.app.console, "format")?;

    let execution = if cli_options.json {
        Execution::with_report(
            TraversalMode::Format {
                ignore_errors: cli_options.skip_errors,
                write,
                stdin,
            },
            ReportMode::Json,
        )
    } else {
        Execution::new(TraversalMode::Format {
            ignore_errors: cli_options.skip_errors,
            write,
            stdin,
        })
    };

    execute_mode(execution, session, &cli_options, paths)
}
