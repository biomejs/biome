use crate::cli_options::CliOptions;
use crate::configuration::{load_configuration, LoadedConfiguration};
use crate::diagnostics::DeprecatedArgument;
use crate::execute::ReportMode;
use crate::vcs::store_path_to_ignore_from_vcs;
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use biome_console::{markup, ConsoleExt};
use biome_diagnostics::PrintDiagnostic;
use biome_service::configuration::json::JsonFormatter;
use biome_service::configuration::vcs::VcsConfiguration;
use biome_service::configuration::{FilesConfiguration, FormatterConfiguration};
use biome_service::workspace::UpdateSettingsParams;
use biome_service::{JavascriptFormatter, MergeWith};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) struct FormatCommandPayload {
    pub(crate) javascript_formatter: Option<JavascriptFormatter>,
    pub(crate) json_formatter: Option<JsonFormatter>,
    pub(crate) formatter_configuration: Option<FormatterConfiguration>,
    pub(crate) vcs_configuration: Option<VcsConfiguration>,
    pub(crate) files_configuration: Option<FilesConfiguration>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) write: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) paths: Vec<OsString>,
}

/// Handler for the "format" command of the Biome CLI
pub(crate) fn format(
    mut session: CliSession,
    payload: FormatCommandPayload,
) -> Result<(), CliDiagnostic> {
    let FormatCommandPayload {
        javascript_formatter,
        formatter_configuration,
        vcs_configuration,
        paths,
        cli_options,
        stdin_file_path,
        files_configuration,
        write,
        json_formatter,
    } = payload;
    let loaded_configuration = load_configuration(&mut session, &cli_options)?.with_file_path();

    loaded_configuration.check_for_errors(session.app.console, cli_options.verbose)?;

    let LoadedConfiguration {
        mut configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;
    if formatter_configuration
        .as_ref()
        .is_some_and(|f| f.indent_size.is_some())
    {
        let console = &mut session.app.console;
        let diagnostic = DeprecatedArgument::new(markup! {
            "The argument "<Emphasis>"--indent-size"</Emphasis>"is deprecated, it will be removed in the next major release. Use "<Emphasis>"--indent-width"</Emphasis>" instead."
        });
        console.error(markup! {
            {PrintDiagnostic::simple(&diagnostic)}
        })
    }

    configuration.merge_with(javascript_formatter);
    configuration.merge_with(json_formatter);
    configuration.merge_with(formatter_configuration);
    configuration.merge_with(vcs_configuration);
    configuration.merge_with(files_configuration);

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    store_path_to_ignore_from_vcs(
        &mut session,
        &mut configuration,
        vcs_base_path,
        &cli_options,
    )?;
    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    let stdin = if let Some(stdin_file_path) = stdin_file_path {
        let console = &mut session.app.console;
        let input_code = console.read();
        if let Some(input_code) = input_code {
            let path = PathBuf::from(stdin_file_path);
            Some((path, input_code))
        } else {
            // we provided the argument without a piped stdin, we bail
            return Err(CliDiagnostic::missing_argument("stdin", "format"));
        }
    } else {
        None
    };

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
