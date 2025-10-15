use std::fmt::{Debug, Formatter};

use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

use biome_console::fmt::Display;
use biome_console::markup;
use biome_deserialize::DeserializationDiagnostic;
use biome_diagnostics::{Diagnostic, Error, MessageAndDescription};
use biome_fs::FileSystemDiagnostic;
use biome_grit_patterns::CompileError;
use biome_resolver::{ResolveError, ResolveErrorDiagnostic};
use biome_rowan::SyntaxError;

/// Series of errors that can be thrown while loading a plugin.
#[derive(Deserialize, Diagnostic, Serialize)]
pub enum PluginDiagnostic {
    /// Thrown when a plugin can't be resolved from `node_modules`.
    CantResolve(CantResolve),

    /// Error compiling the plugin
    Compile(CompileDiagnostic),

    /// Error thrown when deserializing a plugin manifest, such as:
    /// - syntax error
    /// - incorrect fields
    /// - incorrect values
    Deserialization(DeserializationDiagnostic),

    /// Error loading the plugin from the file system.
    FileSystem(FileSystemDiagnostic),

    /// When something is wrong with the manifest.
    InvalidManifest(InvalidManifest),

    /// When an analyzer rule plugin uses an unsupported file format.
    UnsupportedRuleFormat(UnsupportedRuleFormat),

    /// When plugin is requested but not loaded
    NotLoaded(NotLoaded),
}

impl From<CompileError> for PluginDiagnostic {
    fn from(value: CompileError) -> Self {
        Self::Compile(CompileDiagnostic {
            message: MessageAndDescription::from(
                markup! {"Failed to compile the Grit plugin"}.to_owned(),
            ),
            source: Some(Error::from(value)),
        })
    }
}

#[cfg(feature = "js_plugin")]
impl From<boa_engine::JsError> for PluginDiagnostic {
    fn from(value: boa_engine::JsError) -> Self {
        Self::Compile(CompileDiagnostic {
            message: MessageAndDescription::from(
                markup! {"Failed to compile the JS plugin: "{value.to_string()}}.to_owned(),
            ),
            source: None,
        })
    }
}

impl From<DeserializationDiagnostic> for PluginDiagnostic {
    fn from(value: DeserializationDiagnostic) -> Self {
        Self::Deserialization(value)
    }
}

impl From<FileSystemDiagnostic> for PluginDiagnostic {
    fn from(value: FileSystemDiagnostic) -> Self {
        Self::FileSystem(value)
    }
}

impl From<SyntaxError> for PluginDiagnostic {
    fn from(_: SyntaxError) -> Self {
        Self::Deserialization(DeserializationDiagnostic::new(markup! {"Syntax Error"}))
    }
}

impl PluginDiagnostic {
    pub fn cant_resolve(path: Utf8PathBuf, kind: Option<ResolveError>) -> Self {
        Self::CantResolve(CantResolve {
            message: MessageAndDescription::from(
                markup! {
                   "Failed to resolve the plugin manifest from "
                   <Emphasis>{path.to_string()}</Emphasis>
                }
                .to_owned(),
            ),
            source: kind.map(|kind| ResolveErrorDiagnostic::new(kind, path).into()),
        })
    }

    pub fn invalid_manifest(message: impl Display, source: Option<Error>) -> Self {
        Self::InvalidManifest(InvalidManifest {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
            source,
        })
    }

    pub fn unsupported_rule_format(message: impl Display) -> Self {
        Self::UnsupportedRuleFormat(UnsupportedRuleFormat {
            message: MessageAndDescription::from(markup! {{message}}.to_owned()),
        })
    }

    pub fn not_loaded(path: Utf8PathBuf) -> Self {
        Self::NotLoaded(NotLoaded {
            message: MessageAndDescription::from(
                markup! {
                    "Plugin is requested but not loaded: "
                    <Emphasis>{path.to_string()}</Emphasis>
                }
                .to_owned(),
            ),
        })
    }
}

impl Debug for PluginDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for PluginDiagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.description(f)
    }
}

impl From<PluginDiagnostic> for biome_diagnostics::serde::Diagnostic {
    fn from(error: PluginDiagnostic) -> Self {
        Self::new(error)
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "plugin",
    severity = Error,
)]
pub struct CompileDiagnostic {
    #[message]
    #[description]
    message: MessageAndDescription,

    #[serde(skip)]
    #[source]
    source: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "plugin",
    severity = Error,
)]
pub struct InvalidManifest {
    #[message]
    #[description]
    message: MessageAndDescription,

    #[serde(skip)]
    #[source]
    source: Option<Error>,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "plugin",
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
#[diagnostic(
    category = "plugin",
    severity = Error,
)]
pub struct UnsupportedRuleFormat {
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "plugin",
    severity = Error,
)]
pub struct NotLoaded {
    #[message]
    #[description]
    pub message: MessageAndDescription,
}

#[cfg(test)]
mod test {
    use crate::plugin_manifest::PluginManifest;

    use biome_deserialize::json::deserialize_from_json_str;
    use biome_diagnostics::{Error, print_diagnostic_to_string};
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
    fn deserialization_error() {
        let content = "{}";
        let result =
            deserialize_from_json_str::<PluginManifest>(content, JsonParserOptions::default(), "");

        assert!(result.has_errors());
        for diagnostic in result.into_diagnostics() {
            snap_diagnostic("deserialization_error", diagnostic)
        }
    }

    #[test]
    fn deserialization_quick_check() {
        let content = r#"{
    "version": 1,
    "rules": [
        "./rules/my-rule.grit"
    ]
}"#;
        let _result =
            deserialize_from_json_str::<PluginManifest>(content, JsonParserOptions::default(), "")
                .into_deserialized()
                .unwrap_or_default();
    }
}
