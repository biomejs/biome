use biome_console::fmt::Display;
use biome_console::{markup, MarkupBuf};
use biome_deserialize::DeserializationDiagnostic;
use biome_diagnostics::ResolveError;
use biome_diagnostics::{Advices, Diagnostic, Error, LogCategory, MessageAndDescription, Visit};
use biome_rowan::SyntaxError;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Series of errors that can be thrown while computing the configuration.
#[derive(Debug, Deserialize, Diagnostic, Serialize)]
pub enum ConfigurationDiagnostic {
    /// Diagnostics related to `biome.json` files
    Biome(BiomeDiagnostic),
    /// Diagnostics related to `.editorconfig` files
    EditorConfig(EditorConfigDiagnostic),
}

impl From<BiomeDiagnostic> for ConfigurationDiagnostic {
    fn from(value: BiomeDiagnostic) -> Self {
        Self::Biome(value)
    }
}

impl From<EditorConfigDiagnostic> for ConfigurationDiagnostic {
    fn from(value: EditorConfigDiagnostic) -> Self {
        Self::EditorConfig(value)
    }
}

/// Series of errors that can be thrown while computing the configuration, specifically for `biome.json`.
#[derive(Deserialize, Diagnostic, Serialize)]
pub enum BiomeDiagnostic {
    /// Thrown when the program can't serialize the configuration, while saving it
    SerializationError(SerializationError),

    NoConfigurationFileFound(NoConfigurationFileFound),

    /// Thrown when trying to **create** a new configuration file, but it exists already
    ConfigAlreadyExists(ConfigAlreadyExists),

    /// Error thrown when de-serialising the configuration from file, the issues can be many:
    /// - syntax error
    /// - incorrect fields
    /// - incorrect values
    Deserialization(DeserializationDiagnostic),

    /// When something is wrong with the configuration
    InvalidConfiguration(InvalidConfiguration),

    /// When a user provide a configuration file path that isn't a JSON/JSONC file
    InvalidConfigurationFile(InvalidConfigurationFile),

    /// Thrown when the pattern inside the `ignore` field errors
    InvalidIgnorePattern(InvalidIgnorePattern),

    /// Thrown when there's something wrong with the files specified inside `"extends"`
    CantLoadExtendFile(CantLoadExtendFile),

    /// Thrown when a configuration file can't be resolved from `node_modules`
    CantResolve(CantResolve),
}

impl From<SyntaxError> for BiomeDiagnostic {
    fn from(_: SyntaxError) -> Self {
        BiomeDiagnostic::Deserialization(DeserializationDiagnostic::new(markup! {"Syntax Error"}))
    }
}

impl From<DeserializationDiagnostic> for BiomeDiagnostic {
    fn from(value: DeserializationDiagnostic) -> Self {
        BiomeDiagnostic::Deserialization(value)
    }
}

impl BiomeDiagnostic {
    pub fn new_serialization_error() -> Self {
        Self::SerializationError(SerializationError)
    }

    pub fn new_invalid_ignore_pattern(
        pattern: impl Into<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self::InvalidIgnorePattern(InvalidIgnorePattern {
            message: format!(
                "Couldn't parse the pattern \"{}\". Reason: {}",
                pattern.into(),
                reason.into()
            ),
            file_path: None,
        })
    }

    pub fn new_invalid_ignore_pattern_with_path(
        pattern: impl std::fmt::Display,
        reason: impl std::fmt::Display,
        file_path: impl Into<String>,
    ) -> Self {
        Self::InvalidIgnorePattern(InvalidIgnorePattern {
            message: format!(
                "Couldn't parse the pattern \"{}\". Reason: {}",
                pattern, reason,
            ),
            file_path: Some(file_path.into()),
        })
    }

    pub fn new_already_exists() -> Self {
        Self::ConfigAlreadyExists(ConfigAlreadyExists {})
    }

    pub fn invalid_configuration(message: impl Display) -> Self {
        Self::InvalidConfiguration(InvalidConfiguration {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
        })
    }

    pub fn invalid_configuration_file(path: &Utf8Path) -> Self {
        Self::InvalidConfigurationFile(InvalidConfigurationFile {
            path: path.to_string(),
        })
    }

    pub fn no_configuration_file_found(path: &Utf8Path) -> Self {
        Self::NoConfigurationFileFound(NoConfigurationFileFound {
            path: path.to_string(),
        })
    }

    pub fn cant_resolve(path: impl Display, source: oxc_resolver::ResolveError) -> Self {
        Self::CantResolve(CantResolve {
            message: MessageAndDescription::from(
                markup! {
                   "Failed to resolve the configuration from "{{path}}
                }
                .to_owned(),
            ),
            source: Some(Error::from(ResolveError::from(source))),
        })
    }
}

impl Debug for BiomeDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for BiomeDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ConfigurationAdvices {
    messages: Vec<MarkupBuf>,
}

impl Advices for ConfigurationAdvices {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        for message in &self.messages {
            visitor.record_log(LogCategory::Info, message)?;
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "Failed to serialize",
    category = "configuration",
    severity = Error
)]
pub struct SerializationError;

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    message = "It seems that a configuration file already exists",
    category = "configuration",
    severity = Error
)]
pub struct ConfigAlreadyExists {}

#[derive(Debug, Diagnostic, Serialize, Deserialize)]
#[diagnostic(
    category = "configuration",
    severity = Error,
    message(
        message("Biome couldn't find a configuration in the directory "<Emphasis>{self.path}</Emphasis>"."),
        description = "Biome couldn't find a configuration in the directory {path}."
    )
)]
pub struct NoConfigurationFileFound {
    #[location(resource)]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct InvalidIgnorePattern {
    #[message]
    #[description]
    pub message: String,

    #[location(resource)]
    pub file_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct CantLoadExtendFile {
    #[location(resource)]
    file_path: String,
    #[message]
    #[description]
    message: MessageAndDescription,

    #[verbose_advice]
    verbose_advice: ConfigurationAdvices,
}

impl CantLoadExtendFile {
    pub fn new(file_path: impl Into<String>, message: impl Display) -> Self {
        Self {
            file_path: file_path.into(),
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
            verbose_advice: ConfigurationAdvices::default(),
        }
    }

    pub fn with_verbose_advice(mut self, messsage: impl Display) -> Self {
        self.verbose_advice
            .messages
            .push(markup! {{messsage}}.to_owned());
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct InvalidConfiguration {
    #[message]
    #[description]
    message: MessageAndDescription,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
    message(
        description = "Invalid configuration file. Expected JSON or JSONC file, but got {path}.",
        message("Invalid configuration file. Expected JSON or JSONC file, but got "{self.path}".")
    )
)]
pub struct InvalidConfigurationFile {
    #[location(resource)]
    path: String,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct CantResolve {
    #[message]
    #[description]
    message: MessageAndDescription,

    #[serde(skip)]
    #[source]
    source: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
pub enum EditorConfigDiagnostic {
    /// Failed to parse the .editorconfig file.
    ParseFailed(ParseFailedDiagnostic),
    /// An option is completely incompatible with biome.
    Incompatible(IncompatibleDiagnostic),
    /// A glob pattern that biome doesn't support.
    UnknownGlobPattern(UnknownGlobPatternDiagnostic),
    /// A glob pattern that contains invalid syntax.
    InvalidGlobPattern(InvalidGlobPatternDiagnostic),
}

impl EditorConfigDiagnostic {
    pub fn incompatible(key: impl Into<String>, message: impl Into<String>) -> Self {
        Self::Incompatible(IncompatibleDiagnostic {
            message: MessageAndDescription::from(
                markup! { "Key '"{key.into()}"' is incompatible with biome: "{message.into()}}
                    .to_owned(),
            ),
        })
    }

    pub fn unknown_glob_pattern(pattern: impl Into<String>) -> Self {
        Self::UnknownGlobPattern(UnknownGlobPatternDiagnostic {
            message: MessageAndDescription::from(
                markup! { "This glob pattern is incompatible with biome: "{pattern.into()}}
                    .to_owned(),
            ),
        })
    }

    pub fn invalid_glob_pattern(pattern: impl Into<String>, reason: impl Into<String>) -> Self {
        Self::InvalidGlobPattern(InvalidGlobPatternDiagnostic {
            message: MessageAndDescription::from(
                markup! { "This glob pattern is invalid: "{pattern.into()}" Reason: "{reason.into()}}
                    .to_owned(),
            ),
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
    message = "Failed to parse the .editorconfig file.",
)]
pub struct ParseFailedDiagnostic {
    #[serde(skip)]
    #[source]
    pub source: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct IncompatibleDiagnostic {
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Warning,
)]
pub struct UnknownGlobPatternDiagnostic {
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
)]
pub struct InvalidGlobPatternDiagnostic {
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

#[cfg(test)]
mod test {
    use crate::{BiomeDiagnostic, Configuration};
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, Error};
    use biome_json_parser::JsonParserOptions;

    fn snap_diagnostic(test_name: &str, diagnostic: Error) {
        let content = print_diagnostic_to_string(&diagnostic);

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }

    #[test]
    fn diagnostic_size() {
        assert_eq!(std::mem::size_of::<BiomeDiagnostic>(), 96);
    }

    #[test]
    fn config_already_exists() {
        snap_diagnostic(
            "config_already_exists",
            BiomeDiagnostic::new_already_exists().with_file_path("biome.json"),
        )
    }

    #[test]
    fn deserialization_error() {
        let content = "{ \n\n\"formatter\" }";
        let result =
            deserialize_from_json_str::<Configuration>(content, JsonParserOptions::default(), "");

        assert!(result.has_errors());
        for diagnostic in result.into_diagnostics() {
            snap_diagnostic("deserialization_error", diagnostic)
        }
    }

    #[test]
    fn deserialization_quick_check() {
        let content = r#"{
  "linter": {
    "rules": {
        "recommended": true,
        "suspicious": {
            "noDebugger": {
                "level": "off",
                "options": { "hooks": [] }
            }
        }
    }
  }
}"#;
        let _result =
            deserialize_from_json_str::<Configuration>(content, JsonParserOptions::default(), "")
                .into_deserialized()
                .unwrap_or_default();
    }
}
