mod configuration;
mod key;

use self::{
    configuration::{ConfigurationInspector, ConfigurationKind, SourceReference, SourceScope},
    key::ConfigurationKey,
};
use super::{InspectSubCommand, validate_configuration_diagnostics};
use crate::{CliDiagnostic, CliSession, cli_options::CliOptions};
use biome_configuration::BiomeDiagnostic;
use biome_console::{Console, ConsoleExt, MarkupBuf, markup};
use biome_deserialize::TextRange;
use biome_diagnostics::{
    Advices, CodeFrameAdvice, Diagnostic, Error, LogCategory, PrintDiagnostic, Visit,
};
use biome_service::{WorkspaceError, configuration::load_configuration, settings::Settings};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Serialize;
use serde_json::Value;
use std::{io, sync::Arc};

/// Dispatches an `inspect` subcommand without mutating the workspace or configuration files.
pub(crate) fn inspect(
    session: CliSession,
    cli_options: &CliOptions,
    sub_command: InspectSubCommand,
) -> Result<(), CliDiagnostic> {
    match sub_command {
        InspectSubCommand::Config { key, path, json } => {
            ConfigInspectionCommand::new(session, cli_options, key, path, json)?.execute()
        }
    }
}

/// Owns the inputs and application state for one `inspect config` invocation.
///
/// Validation happens during construction so `execute` can run the inspection pipeline without
/// representing invalid combinations such as `--path` without a configuration key.
struct ConfigInspectionCommand<'app, 'options> {
    session: CliSession<'app>,
    cli_options: &'options CliOptions,
    key: Option<ConfigurationKey>,
    path: Option<String>,
    json: bool,
}

impl<'app, 'options> ConfigInspectionCommand<'app, 'options> {
    /// Parses the optional key and rejects argument combinations that cannot be executed.
    fn new(
        session: CliSession<'app>,
        cli_options: &'options CliOptions,
        key: Option<String>,
        path: Option<String>,
        json: bool,
    ) -> Result<Self, CliDiagnostic> {
        let key = key.map(ConfigurationKey::parse).transpose()?;

        if path.is_some() && key.is_none() {
            return Err(CliDiagnostic::missing_argument("KEY", "inspect config"));
        }

        Ok(Self {
            session,
            cli_options,
            key,
            path,
            json,
        })
    }

    /// Loads the retained configuration graph and renders either its resolved structure or one
    /// effective key lookup.
    fn execute(self) -> Result<(), CliDiagnostic> {
        let Self {
            session,
            cli_options,
            key,
            path,
            json,
        } = self;

        let fs = session.app.workspace.fs();
        let working_directory = fs.working_directory().unwrap_or_default();
        let path_hint = cli_options.as_configuration_path_hint(&working_directory);
        let loaded_configuration = load_configuration(fs, path_hint)?;
        validate_configuration_diagnostics(
            &loaded_configuration,
            session.app.console,
            cli_options.verbose,
        )?;

        let configuration_path = loaded_configuration.file_path().map(Utf8Path::to_path_buf);
        let configuration_source = Arc::new(loaded_configuration.source);
        let mut settings = Settings::default();
        settings.merge_with_configuration_source(configuration_source.clone())?;
        let inspector =
            ConfigurationInspector::new(&configuration_source).map_err(|diagnostics| {
                Self::report_source_diagnostics(
                    session.app.console,
                    diagnostics,
                    cli_options.verbose,
                )
            })?;

        let Some(key) = key else {
            return Self::print_resolved_configuration(
                session,
                json,
                configuration_path.as_deref(),
                &inspector,
            );
        };

        let matched_path = path.as_deref().map(|path| {
            let path = Utf8PathBuf::from(path);
            let path = path.strip_prefix("./").unwrap_or(&path);
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                working_directory.join(path)
            }
        });
        let display_path = matched_path.as_deref().map(|path| {
            configuration_source
                .directory_path
                .as_deref()
                .and_then(|directory| path.strip_prefix(directory).ok())
                .unwrap_or(path)
                .to_string()
        });
        let matching_overrides = matched_path.as_deref().map_or_else(Vec::new, |path| {
            settings
                .override_settings
                .matching_indices(path)
                .collect::<Vec<_>>()
        });
        let inspection =
            inspector.inspect_key(&key, &matching_overrides, display_path.as_deref())?;

        if json {
            let output = InspectionJson {
                key: key.as_str(),
                value: inspection.value.as_ref().unwrap_or(&Value::Null),
                source: inspection.source_json(),
            };
            let output = serde_json::to_string_pretty(&output)
                .map_err(|_| WorkspaceError::from(BiomeDiagnostic::new_serialization_error()))?;
            session.app.console.log(markup! {{output}});
            return Ok(());
        }

        let diagnostic = inspection.value.as_ref().map_or_else(
            || InspectionDiagnostic::absent(key.as_str().to_string()),
            |value| {
                InspectionDiagnostic::configured(
                    key.as_str().to_string(),
                    value,
                    &inspection.sources,
                    configuration_path.as_deref(),
                )
            },
        );
        session
            .app
            .console
            .log(markup! {{PrintDiagnostic::simple(&diagnostic)}});
        Ok(())
    }

    /// Renders the structurally resolved configuration without evaluating overrides for a path.
    fn print_resolved_configuration(
        session: CliSession,
        json: bool,
        configuration_path: Option<&Utf8Path>,
        inspector: &ConfigurationInspector,
    ) -> Result<(), CliDiagnostic> {
        let output = serde_json::to_string_pretty(inspector.serialized_configuration())
            .map_err(|_| WorkspaceError::from(BiomeDiagnostic::new_serialization_error()))?;
        if json {
            session.app.console.log(markup! {{output}});
            return Ok(());
        }

        let configuration_paths = inspector.configuration_paths().collect::<Vec<_>>();
        let diagnostic = InspectionDiagnostic::resolved(
            configuration_path,
            &configuration_paths,
            output,
            inspector.has_overrides(),
        );
        session
            .app
            .console
            .log(markup! {{PrintDiagnostic::simple(&diagnostic)}});
        Ok(())
    }

    /// Prints every diagnostic associated with a retained source before returning the command-level
    /// failure diagnostic.
    fn report_source_diagnostics(
        console: &mut dyn Console,
        diagnostics: Vec<Error>,
        verbose: bool,
    ) -> CliDiagnostic {
        for diagnostic in diagnostics {
            if diagnostic.tags().is_verbose() {
                if verbose {
                    console.error(markup! {{PrintDiagnostic::verbose(&diagnostic)}});
                }
            } else {
                console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}});
            }
        }
        CliDiagnostic::workspace_error(
            BiomeDiagnostic::invalid_configuration(
                "Biome exited because the retained configuration source could not be inspected.",
            )
            .into(),
        )
    }
}

/// Stable JSON response for a configuration-key inspection.
///
/// `source` is null when the key is not configured and contains either one declaration or an
/// ordered set of contributors for a composite value.
#[derive(Serialize)]
struct InspectionJson<'a> {
    key: &'a str,
    value: &'a Value,
    source: Option<Value>,
}

/// Human-readable result of a successful configuration inspection.
///
/// Configured values can attach retained source text and a value range, allowing the diagnostic
/// renderer to produce the same code frame style as other Biome diagnostics.
#[derive(Debug, Diagnostic)]
#[diagnostic(category = "configuration", severity = Information)]
struct InspectionDiagnostic {
    #[message]
    message: MarkupBuf,
    #[location(resource)]
    path: Option<String>,
    #[location(span)]
    span: Option<TextRange>,
    #[location(source_code)]
    source_code: Option<String>,
    #[advice]
    advice: InspectionAdvice,
}

impl InspectionDiagnostic {
    fn resolved(
        path: Option<&Utf8Path>,
        configuration_paths: &[&Utf8Path],
        output: String,
        has_overrides: bool,
    ) -> Self {
        let path_text =
            path.map_or_else(|| "default configuration".to_string(), Utf8Path::to_string);
        let mut advice = Vec::new();
        if !configuration_paths.is_empty() {
            let mut paths = String::from("Configuration files used (merge order):");
            for path in configuration_paths {
                paths.push_str("\n  - ");
                paths.push_str(path.as_str());
            }
            advice.push(AdviceLine::Plain(markup! {{paths}}.to_owned()));
        }
        advice.push(AdviceLine::Plain(markup! {{output}}.to_owned()));
        if has_overrides {
            advice.push(AdviceLine::Info(
                markup! { "Overrides are included but were not evaluated because no "<Emphasis>"--path"</Emphasis>" was provided." }.to_owned(),
            ));
        }
        Self {
            message: markup! { "Resolved configuration from "<Emphasis>{path_text}</Emphasis>"." }
                .to_owned(),
            path: path.map(Utf8Path::to_string),
            span: None,
            source_code: None,
            advice: InspectionAdvice(advice),
        }
    }

    fn absent(key: String) -> Self {
        Self {
            message: markup! { "The key "<Emphasis>{key}</Emphasis>" doesn't exist." }.to_owned(),
            path: None,
            span: None,
            source_code: None,
            advice: InspectionAdvice(vec![AdviceLine::Info(
                markup! { "Make sure it's correctly spelled." }.to_owned(),
            )]),
        }
    }

    /// Builds a configured-value diagnostic using the last contributor as its primary location.
    ///
    /// The advice identifies composite values as assembled from multiple sources.
    fn configured(
        key: String,
        value: &Value,
        sources: &[SourceReference],
        root_path: Option<&Utf8Path>,
    ) -> Self {
        let value = Self::display_value(value);
        let location = if sources.len() == 1 {
            sources.first().map(|source| {
                (
                    source.path.to_string(),
                    source.range,
                    source.source.to_string(),
                )
            })
        } else {
            None
        };
        let advice = Self::source_advices(sources, root_path);
        Self {
            message: markup! {
                "The key "<Emphasis>{key}</Emphasis>" has the value "<Emphasis>{value}</Emphasis>"."
            }
            .to_owned(),
            path: location.as_ref().map(|(path, _, _)| path.clone()),
            span: location.as_ref().and_then(|(_, span, _)| *span),
            source_code: location.map(|(_, _, source)| source),
            advice: InspectionAdvice(advice),
        }
    }

    fn display_value(value: &Value) -> String {
        match value {
            Value::String(value) => value.clone(),
            _ => serde_json::to_string(value).unwrap_or_else(|_| "<unknown>".to_string()),
        }
    }

    /// Describes every contributing declaration for merged values and the effective declaration
    /// for scalar values.
    fn source_advices(
        sources: &[SourceReference],
        root_path: Option<&Utf8Path>,
    ) -> Vec<AdviceLine> {
        if sources.len() > 1 {
            let source_count = sources.len();
            let mut advice = vec![AdviceLine::Info(
                markup! {
                    "This value is defined across "<Emphasis>{source_count}</Emphasis>
                    " configuration files."
                }
                .to_owned(),
            )];
            for source in sources {
                advice.push(Self::composite_source_advice(source));
                advice.push(AdviceLine::Frame {
                    path: source.path.to_string(),
                    span: source.range,
                    source_code: source.source.to_string(),
                });
                if let Some(match_advice) = Self::override_match_advice(source) {
                    advice.push(match_advice);
                }
            }
            return advice;
        }
        let Some(source) = sources.first() else {
            return Vec::new();
        };
        let mut advice = Vec::new();
        match source.scope {
            SourceScope::Base => match source.kind {
                ConfigurationKind::Root => advice.push(AdviceLine::Info(
                    markup! { "This value is provided by the root configuration." }.to_owned(),
                )),
                ConfigurationKind::Extend { .. } => {
                    let root_path = root_path.map_or("the root configuration", Utf8Path::as_str);
                    advice.push(AdviceLine::Info(
                        markup! {
                            "This value comes from "<Emphasis>{source.path.as_str()}</Emphasis>
                            ", extended by "<Emphasis>{root_path}</Emphasis>"."
                        }
                        .to_owned(),
                    ));
                }
            },
            SourceScope::Override => {
                let index = source.override_index.unwrap_or_default();
                let kind = match source.kind {
                    ConfigurationKind::Root => "root configuration",
                    ConfigurationKind::Extend { .. } => "extended configuration",
                };
                advice.push(AdviceLine::Info(
                    markup! {
                        "This value is provided by override "<Emphasis>{index}</Emphasis>
                        " in the "<Emphasis>{kind}</Emphasis>"."
                    }
                    .to_owned(),
                ));
                if let Some(match_advice) = Self::override_match_advice(source) {
                    advice.push(match_advice);
                }
            }
        }
        advice
    }

    /// Introduces one merged-value code frame with its configuration path and scope.
    fn composite_source_advice(source: &SourceReference) -> AdviceLine {
        let path = source.path.as_str();
        let index = source.override_index.unwrap_or_default();
        let text = match (source.scope, source.kind) {
            (SourceScope::Base, ConfigurationKind::Root) => markup! {
                "The highlighted value is configured in the root configuration "
                <Emphasis>{path}</Emphasis>"."
            }
            .to_owned(),
            (SourceScope::Base, ConfigurationKind::Extend { specifier }) => {
                if let Some(specifier) = specifier.as_deref() {
                    markup! {
                        "The highlighted value is configured in "<Emphasis>{path}</Emphasis>
                        ", referenced by "<Emphasis>"extends"</Emphasis>" as "
                        <Emphasis>{specifier}</Emphasis>"."
                    }
                    .to_owned()
                } else {
                    markup! {
                        "The highlighted value is configured in "<Emphasis>{path}</Emphasis>
                        ", referenced by "<Emphasis>"extends"</Emphasis>"."
                    }
                    .to_owned()
                }
            }
            (SourceScope::Override, ConfigurationKind::Root) => markup! {
                "The highlighted value is configured by override "<Emphasis>{index}</Emphasis>
                " in "<Emphasis>{path}</Emphasis>"."
            }
            .to_owned(),
            (SourceScope::Override, ConfigurationKind::Extend { specifier }) => {
                if let Some(specifier) = specifier.as_deref() {
                    markup! {
                        "The highlighted value is configured by override "
                        <Emphasis>{index}</Emphasis>" in "<Emphasis>{path}</Emphasis>
                        ", referenced by "<Emphasis>"extends"</Emphasis>" as "
                        <Emphasis>{specifier}</Emphasis>"."
                    }
                    .to_owned()
                } else {
                    markup! {
                        "The highlighted value is configured by override "
                        <Emphasis>{index}</Emphasis>" in "<Emphasis>{path}</Emphasis>
                        ", referenced by "<Emphasis>"extends"</Emphasis>"."
                    }
                    .to_owned()
                }
            }
        };
        AdviceLine::Info(text)
    }

    /// Explains why an override contributor applies to the inspected path.
    fn override_match_advice(source: &SourceReference) -> Option<AdviceLine> {
        let path = source.matched_path.as_deref()?;
        let includes = match source.includes.as_deref() {
            Some([include]) => include.clone(),
            Some(includes) => serde_json::to_string(includes).unwrap_or_else(|_| "[]".to_string()),
            None => "[]".to_string(),
        };
        Some(AdviceLine::Info(
            markup! {
                "The override matched "<Emphasis>{path}</Emphasis>
                " using "<Emphasis>{includes}</Emphasis>"."
            }
            .to_owned(),
        ))
    }
}

/// One ordered advice entry rendered as either a log line or a source code frame.
#[derive(Debug)]
enum AdviceLine {
    Frame {
        path: String,
        span: Option<TextRange>,
        source_code: String,
    },
    Info(MarkupBuf),
    Plain(MarkupBuf),
}

/// Ordered supplemental output rendered after the inspection message and optional code frame.
#[derive(Debug)]
struct InspectionAdvice(Vec<AdviceLine>);

impl Advices for InspectionAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        for line in &self.0 {
            match line {
                AdviceLine::Frame {
                    path,
                    span,
                    source_code,
                } => CodeFrameAdvice {
                    path,
                    span,
                    source_code,
                }
                .record(visitor)?,
                AdviceLine::Info(line) => visitor.record_log(LogCategory::Info, line)?,
                AdviceLine::Plain(line) => visitor.record_log(LogCategory::None, line)?,
            }
        }
        Ok(())
    }
}
