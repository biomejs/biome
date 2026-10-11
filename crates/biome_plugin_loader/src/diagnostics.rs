use std::fmt::{Debug, Formatter};
use std::io;

use camino::{Utf8Path, Utf8PathBuf};
use serde::{Deserialize, Serialize};

use biome_console::fmt::Display;
use biome_console::markup;
use biome_deserialize::DeserializationDiagnostic;
use biome_diagnostics::{Advices, Diagnostic, Error, LogCategory, MessageAndDescription, Visit};
use biome_fs::{FileSystemDiagnostic, ManifestName};
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

    /// Error thrown when deserializing a Biome manifest, such as:
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
    pub fn cant_read_plugin_file(path: Utf8PathBuf, source: FileSystemDiagnostic) -> Self {
        Self::InvalidManifest(InvalidManifest {
            message: MessageAndDescription::from(
                markup! {
                    "Biome couldn't read the plugin file "
                    <Emphasis>{path.to_string()}</Emphasis>"."
                }
                .to_owned(),
            ),
            source: Some(Error::from(source)),
        })
    }

    /// Reports a plugin or manifest rule path that doesn't point to a file.
    pub fn plugin_file_not_found(path: &Utf8Path) -> Self {
        Self::CantResolve(CantResolve {
            message: MessageAndDescription::from(
                markup! {
                    "Biome couldn't find the plugin file "<Emphasis>{path.as_str()}</Emphasis>"."
                }
                .to_owned(),
            ),
            advice: PackageLookupAdvice::default(),
            source: None,
        })
    }

    /// Reports a plugin path that doesn't contain a Biome manifest.
    pub fn missing_manifest(path: &Utf8Path) -> Self {
        Self::CantResolve(CantResolve {
            message: MessageAndDescription::from(
                markup! {
                    "Biome couldn't find a "<Emphasis>{ManifestName::biome_manifest_json()}</Emphasis>
                    " or "<Emphasis>{ManifestName::biome_manifest_jsonc()}</Emphasis>
                    " file in "<Emphasis>{path.as_str()}</Emphasis>"."
                }
                .to_owned(),
            ),
            advice: PackageLookupAdvice::default(),
            source: None,
        })
    }

    /// Reports a plugin package that can't be resolved from `base_path`.
    ///
    /// When the package isn't installed, the diagnostic names the directory where the
    /// `node_modules` lookup starts. Other resolution failures keep the resolver error as
    /// their source.
    pub fn cant_resolve_package(package: &str, base_path: &Utf8Path, kind: ResolveError) -> Self {
        if kind == ResolveError::NotFound {
            return Self::CantResolve(CantResolve {
                message: MessageAndDescription::from(
                    markup! {
                        "Biome couldn't find the plugin package "<Emphasis>{package}</Emphasis>"."
                    }
                    .to_owned(),
                ),
                advice: PackageLookupAdvice {
                    base_path: Some(base_path.to_string()),
                },
                source: None,
            });
        }

        Self::CantResolve(CantResolve {
            message: MessageAndDescription::from(
                markup! {
                    "Biome couldn't resolve the plugin package "<Emphasis>{package}</Emphasis>"."
                }
                .to_owned(),
            ),
            advice: PackageLookupAdvice::default(),
            source: Some(ResolveErrorDiagnostic::new(kind, Utf8PathBuf::from(package)).into()),
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

    #[advice]
    advice: PackageLookupAdvice,

    #[serde(skip)]
    #[source]
    source: Option<Error>,
}

/// Tells the user where Biome looked for a plugin package that isn't installed.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct PackageLookupAdvice {
    /// The directory where the `node_modules` lookup starts. `None` records no advice.
    base_path: Option<String>,
}

impl Advices for PackageLookupAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        let Some(base_path) = &self.base_path else {
            return Ok(());
        };
        // An empty base path is relative, so the lookup starts in the working directory.
        let directory = if base_path.is_empty() {
            markup! { "the working directory" }
        } else {
            markup! { <Emphasis>{base_path}</Emphasis> }
        };
        visitor.record_log(
            LogCategory::Info,
            &markup! {
                "Make sure the package is installed. Biome looks for it in the "
                <Emphasis>"node_modules"</Emphasis>" directory of "{directory}
                " and of each parent directory."
            },
        )
    }
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
    use crate::test_utils::snapshot_content;

    use biome_deserialize::json::deserialize_from_json_str;
    use biome_diagnostics::{Error, print_diagnostic_to_string};
    use biome_json_parser::JsonParserOptions;
    use biome_manifest::BiomeManifest;

    fn snap_diagnostic(test_name: &str, plugin_sources: &[(&str, &str)], diagnostic: Error) {
        let content = print_diagnostic_to_string(&diagnostic);
        let content = snapshot_content(plugin_sources, &[], &content);

        insta::with_settings!({
            prepend_module_to_snapshot => false,
        }, {
            insta::assert_snapshot!(test_name, content);
        });
    }

    #[test]
    fn deserialization_error() {
        let content = r#"{
            "plugins": {
                "rules": [{ "one": "rules/1.grit" }],
                "presets": { "recommended": ["one"] }
            }
        }"#;
        let result =
            deserialize_from_json_str::<BiomeManifest>(content, JsonParserOptions::default(), "");

        assert!(result.has_errors());
        for diagnostic in result.into_diagnostics() {
            snap_diagnostic(
                "deserialization_error",
                &[("biome-manifest.jsonc", content)],
                diagnostic,
            )
        }
    }

    #[test]
    fn deserialization_quick_check() {
        let content = r#"{
    "version": 1,
    "plugins": {
        "rules": [{ "myRule": "./rules/my-rule.grit" }],
        "presets": { "recommended": ["myRule"] }
    }
}"#;
        let _result =
            deserialize_from_json_str::<BiomeManifest>(content, JsonParserOptions::default(), "")
                .into_deserialized()
                .unwrap_or_default();
    }
}
