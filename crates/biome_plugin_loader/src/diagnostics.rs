use biome_console::fmt::Display;
use biome_console::markup;
use biome_deserialize::DeserializationDiagnostic;
use biome_diagnostics::ResolveError;
use biome_diagnostics::{Diagnostic, Error, MessageAndDescription};
use biome_fs::FileSystemDiagnostic;
use biome_grit_patterns::CompileError;
use biome_rowan::SyntaxError;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

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
}

impl From<CompileError> for PluginDiagnostic {
    fn from(value: CompileError) -> Self {
        println!("Compile Error: {value:?}");
        PluginDiagnostic::Compile(CompileDiagnostic {
            source: Some(Error::from(value)),
        })
    }
}

impl From<DeserializationDiagnostic> for PluginDiagnostic {
    fn from(value: DeserializationDiagnostic) -> Self {
        PluginDiagnostic::Deserialization(value)
    }
}

impl From<FileSystemDiagnostic> for PluginDiagnostic {
    fn from(value: FileSystemDiagnostic) -> Self {
        PluginDiagnostic::FileSystem(value)
    }
}

impl From<SyntaxError> for PluginDiagnostic {
    fn from(_: SyntaxError) -> Self {
        PluginDiagnostic::Deserialization(DeserializationDiagnostic::new(markup! {"Syntax Error"}))
    }
}

impl PluginDiagnostic {
    pub fn cant_resolve(path: impl Display, source: Option<ResolveError>) -> Self {
        Self::CantResolve(CantResolve {
            message: MessageAndDescription::from(
                markup! {
                   "Failed to resolve the plugin manifest from "<Emphasis>{path}</Emphasis>
                }
                .to_owned(),
            ),
            source: source.map(Error::from),
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
        biome_diagnostics::serde::Diagnostic::new(error)
    }
}

#[derive(Debug, Serialize, Deserialize, Diagnostic)]
#[diagnostic(
    category = "plugin",
    message = "Error compiling plugin",
    severity = Error,
)]
pub struct CompileDiagnostic {
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

#[cfg(test)]
mod test {
    use crate::plugin_manifest::PluginManifest;

    use biome_deserialize::json::deserialize_from_json_str;
    use biome_diagnostics::{print_diagnostic_to_string, Error};
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
