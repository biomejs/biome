use crate::cli_options::CliOptions;
use crate::commands::validate_configuration_diagnostics;
use crate::diagnostics::DeprecatedArgument;
use crate::execute::ReportMode;
use crate::vcs::retrieve_gitignore_matches;
use crate::{
    execute_mode, setup_cli_subscriber, CliDiagnostic, CliSession, Execution, TraversalMode,
};
use biome_console::{markup, ConsoleExt};
use biome_diagnostics::PrintDiagnostic;
use biome_service::configuration::json::JsonFormatter;
use biome_service::configuration::vcs::VcsConfiguration;
use biome_service::configuration::{
    load_configuration, FilesConfiguration, FormatterConfiguration, LoadedConfiguration,
};
use biome_service::workspace::UpdateSettingsParams;
use biome_service::{ConfigurationBasePath, JavascriptFormatter, MergeWith};
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
    setup_cli_subscriber(cli_options.log_level.clone(), cli_options.log_kind.clone());

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
        mut configuration,
        directory_path: configuration_path,
        ..
    } = loaded_configuration;
    // TODO: remove in biome 2.0
    if formatter_configuration
        .as_ref()
        .is_some_and(|f| f.indent_size.is_some())
    {
        let console = &mut session.app.console;
        let diagnostic = DeprecatedArgument::new(markup! {
            "The argument "<Emphasis>"--indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--indent-width"</Emphasis>" instead."
        });
        console.error(markup! {
            {PrintDiagnostic::simple(&diagnostic)}
        })
    }
    // TODO: remove in biome 2.0
    if javascript_formatter
        .as_ref()
        .is_some_and(|f| f.indent_size.is_some())
    {
        let console = &mut session.app.console;
        let diagnostic = DeprecatedArgument::new(markup! {
            "The argument "<Emphasis>"--javascript-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--javascript-formatter-indent-width"</Emphasis>" instead."
        });
        console.error(markup! {
            {PrintDiagnostic::simple(&diagnostic)}
        })
    }
    // TODO: remove in biome 2.0
    if json_formatter
        .as_ref()
        .is_some_and(|f| f.indent_size.is_some())
    {
        let console = &mut session.app.console;
        let diagnostic = DeprecatedArgument::new(markup! {
            "The argument "<Emphasis>"--json-formatter-indent-size"</Emphasis>" is deprecated, it will be removed in the next major release. Use "<Emphasis>"--json-formatter-indent-width"</Emphasis>" instead."
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
