//! This module contains the configuration of `biome.json`
//!
//! The configuration is divided by "tool", and then it's possible to further customise it
//! by language. The language might further options divided by tool.
pub mod diagnostics;
pub mod formatter;
mod generated;
pub mod javascript;
pub mod json;
pub mod linter;
mod merge;
pub mod organize_imports;
mod overrides;
mod parse;
pub mod vcs;

use crate::configuration::diagnostics::CantLoadExtendFile;
pub use crate::configuration::diagnostics::ConfigurationDiagnostic;
pub(crate) use crate::configuration::generated::push_to_analyzer_rules;
use crate::configuration::json::JsonFormatter;
pub use crate::configuration::merge::MergeWith;
use crate::configuration::organize_imports::{organize_imports, OrganizeImports};
use crate::configuration::overrides::Overrides;
use crate::configuration::vcs::{vcs_configuration, VcsConfiguration};
use crate::diagnostics::{DisabledVcs, VcsDiagnostic};
use crate::settings::WorkspaceSettings;
use crate::{DynRef, WorkspaceError, VERSION};
use biome_analyze::AnalyzerRules;
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, StringSet};
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_fs::{AutoSearchResult, FileSystem, OpenOptions};
use biome_js_analyze::metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use bpaf::Bpaf;
pub use formatter::{
    deserialize_line_width, formatter_configuration, serialize_line_width, FormatterConfiguration,
    PlainIndentStyle,
};
pub use javascript::{javascript_configuration, JavascriptConfiguration, JavascriptFormatter};
pub use json::{json_configuration, JsonConfiguration};
pub use linter::{linter_configuration, LinterConfiguration, RuleConfiguration, Rules};
pub use overrides::to_override_settings;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::ErrorKind;
use std::iter::FusedIterator;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};

/// The configuration that is contained inside the file `biome.json`
#[derive(Debug, Deserialize, Serialize, Clone, Bpaf, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Configuration {
    /// A field for the [JSON schema](https://json-schema.org/) specification
    #[serde(rename(serialize = "$schema", deserialize = "$schema"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub schema: Option<String>,

    /// The configuration of the VCS integration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(vcs_configuration), optional, hide_usage)]
    pub vcs: Option<VcsConfiguration>,

    /// The configuration of the filesystem
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(files_configuration), optional, hide_usage)]
    pub files: Option<FilesConfiguration>,

    /// The configuration of the formatter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(formatter_configuration), optional)]
    pub formatter: Option<FormatterConfiguration>,

    /// The configuration of the import sorting
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external, optional)]
    pub organize_imports: Option<OrganizeImports>,

    /// The configuration for the linter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(linter_configuration), optional)]
    pub linter: Option<LinterConfiguration>,

    /// Specific configuration for the JavaScript language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_configuration), optional)]
    pub javascript: Option<JavascriptConfiguration>,

    /// Specific configuration for the Json language
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(json_configuration), optional)]
    pub json: Option<JsonConfiguration>,

    /// A list of paths to other JSON files, used to extends the current configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub extends: Option<StringSet>,

    /// A list of granular patterns that should be applied only to a sub set of files
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub overrides: Option<Overrides>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            files: None,
            linter: Some(LinterConfiguration {
                enabled: Some(true),
                ..LinterConfiguration::default()
            }),
            organize_imports: Some(OrganizeImports::default()),
            formatter: None,
            javascript: None,
            schema: None,
            vcs: None,
            extends: None,
            json: None,
            overrides: None,
        }
    }
}

impl MergeWith<Configuration> for Configuration {
    fn merge_with(&mut self, other_configuration: Configuration) {
        // files
        self.merge_with(other_configuration.files);
        // formatter
        self.merge_with(other_configuration.formatter);
        // javascript
        self.merge_with(other_configuration.javascript);
        // linter
        self.merge_with(other_configuration.linter);
        // organize imports
        self.merge_with(other_configuration.organize_imports);
        // VCS
        self.merge_with(other_configuration.vcs);
        // overrides
        self.merge_with(other_configuration.overrides);
    }

    fn merge_with_if_not_default(&mut self, other_configuration: Configuration)
    where
        Configuration: Default,
    {
        // files
        self.merge_with_if_not_default(other_configuration.files);
        // formatter
        self.merge_with_if_not_default(other_configuration.formatter);
        // javascript
        self.merge_with_if_not_default(other_configuration.javascript);
        // linter
        self.merge_with_if_not_default(other_configuration.linter);
        // organize imports
        self.merge_with_if_not_default(other_configuration.organize_imports);
        // VCS
        self.merge_with_if_not_default(other_configuration.vcs);
        // overrides
        self.merge_with_if_not_default(other_configuration.overrides);
    }
}

impl Configuration {
    pub fn is_formatter_disabled(&self) -> bool {
        self.formatter
            .as_ref()
            .map(|f| f.is_disabled())
            .unwrap_or(false)
    }

    pub fn is_linter_disabled(&self) -> bool {
        self.linter
            .as_ref()
            .map(|f| f.is_disabled())
            .unwrap_or(false)
    }

    pub fn is_organize_imports_disabled(&self) -> bool {
        self.organize_imports
            .as_ref()
            .map(|f| f.is_disabled())
            .unwrap_or(false)
    }

    pub fn is_vcs_disabled(&self) -> bool {
        self.vcs.as_ref().map(|f| f.is_disabled()).unwrap_or(true)
    }

    pub fn retrieve_gitignore_matches(
        &self,
        file_system: &DynRef<'_, dyn FileSystem>,
        vcs_base_path: Option<&Path>,
    ) -> Result<Vec<String>, WorkspaceError> {
        let Some(vcs) = &self.vcs else {
            return Ok(vec![]);
        };
        if vcs.is_enabled() {
            let vcs_base_path = match (vcs_base_path, &vcs.root) {
                (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
                (None, Some(root)) => PathBuf::from(root),
                (Some(vcs_base_path), None) => PathBuf::from(vcs_base_path),
                (None, None) => {
                    return Err(VcsDiagnostic::DisabledVcs(DisabledVcs {}).into());
                }
            };

            if let Some(client_kind) = &vcs.client_kind {
                if !vcs.ignore_file_disabled() {
                    let result = file_system
                        .auto_search(vcs_base_path, client_kind.ignore_file(), false)
                        .map_err(WorkspaceError::from)?;

                    if let Some(result) = result {
                        return Ok(result
                            .content
                            .lines()
                            .map(String::from)
                            .collect::<Vec<String>>());
                    }
                }
            }
        }
        Ok(vec![])
    }
}

impl MergeWith<Option<VcsConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<VcsConfiguration>) {
        if let Some(other_vcs) = other {
            let vcs = self.vcs.get_or_insert_with(VcsConfiguration::default);
            vcs.merge_with(other_vcs);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<VcsConfiguration>)
    where
        Option<VcsConfiguration>: Default,
    {
        self.merge_with(other)
    }
}

impl MergeWith<Option<OrganizeImports>> for Configuration {
    fn merge_with(&mut self, other: Option<OrganizeImports>) {
        if let Some(other_organize_imports) = other {
            let organize_imports = self
                .organize_imports
                .get_or_insert_with(OrganizeImports::default);
            organize_imports.merge_with(other_organize_imports);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<OrganizeImports>)
    where
        Option<OrganizeImports>: Default,
    {
        self.merge_with(other)
    }
}

impl MergeWith<Option<LinterConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<LinterConfiguration>) {
        if let Some(other_linter) = other {
            let linter = self.linter.get_or_insert_with(LinterConfiguration::default);
            linter.merge_with(other_linter);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<LinterConfiguration>)
    where
        Option<LinterConfiguration>: Default,
    {
        if let Some(other_linter) = other {
            let linter = self.linter.get_or_insert_with(LinterConfiguration::default);
            linter.merge_with_if_not_default(other_linter);
        }
    }
}
impl MergeWith<Option<FilesConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<FilesConfiguration>) {
        if let Some(files_configuration) = other {
            let files = self.files.get_or_insert_with(FilesConfiguration::default);
            files.merge_with(files_configuration);
        };
    }

    fn merge_with_if_not_default(&mut self, other: Option<FilesConfiguration>)
    where
        Option<FilesConfiguration>: Default,
    {
        if let Some(files_configuration) = other {
            let files = self.files.get_or_insert_with(FilesConfiguration::default);
            files.merge_with_if_not_default(files_configuration);
        };
    }
}
impl MergeWith<Option<JavascriptConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<JavascriptConfiguration>) {
        if let Some(other) = other {
            let js_configuration = self
                .javascript
                .get_or_insert_with(JavascriptConfiguration::default);
            js_configuration.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<JavascriptConfiguration>)
    where
        Option<JavascriptConfiguration>: Default,
    {
        if let Some(other) = other {
            let js_configuration = self
                .javascript
                .get_or_insert_with(JavascriptConfiguration::default);
            js_configuration.merge_with_if_not_default(other);
        }
    }
}
impl MergeWith<Option<FormatterConfiguration>> for Configuration {
    fn merge_with(&mut self, other: Option<FormatterConfiguration>) {
        if let Some(other_formatter) = other {
            let formatter = self
                .formatter
                .get_or_insert_with(FormatterConfiguration::default);
            formatter.merge_with(other_formatter);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<FormatterConfiguration>)
    where
        Option<FormatterConfiguration>: Default,
    {
        if let Some(other_formatter) = other {
            let formatter = self
                .formatter
                .get_or_insert_with(FormatterConfiguration::default);
            formatter.merge_with_if_not_default(other_formatter);
        }
    }
}

impl MergeWith<Option<JavascriptFormatter>> for Configuration {
    fn merge_with(&mut self, other: Option<JavascriptFormatter>) {
        let javascript_configuration = self
            .javascript
            .get_or_insert_with(JavascriptConfiguration::default);
        javascript_configuration.merge_with(other);
    }

    fn merge_with_if_not_default(&mut self, other: Option<JavascriptFormatter>)
    where
        Option<JavascriptFormatter>: Default,
    {
        let javascript_configuration = self
            .javascript
            .get_or_insert_with(JavascriptConfiguration::default);
        javascript_configuration.merge_with_if_not_default(other);
    }
}

impl MergeWith<Option<JsonFormatter>> for Configuration {
    fn merge_with(&mut self, other: Option<JsonFormatter>) {
        let javascript_configuration = self.json.get_or_insert_with(JsonConfiguration::default);
        javascript_configuration.merge_with(other);
    }

    fn merge_with_if_not_default(&mut self, other: Option<JsonFormatter>)
    where
        Option<JsonFormatter>: Default,
    {
        let javascript_configuration = self.json.get_or_insert_with(JsonConfiguration::default);
        javascript_configuration.merge_with_if_not_default(other);
    }
}

impl MergeWith<Option<Overrides>> for Configuration {
    fn merge_with(&mut self, other: Option<Overrides>) {
        if let Some(other) = other {
            let overrides = self.overrides.get_or_insert_with(Overrides::default);
            overrides.merge_with(other);
        }
    }

    fn merge_with_if_not_default(&mut self, other: Option<Overrides>)
    where
        Option<Overrides>: Default,
    {
        self.merge_with(other)
    }
}

/// The configuration of the filesystem
#[derive(Default, Debug, Deserialize, Serialize, Clone, Bpaf, Eq, PartialEq)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FilesConfiguration {
    /// The maximum allowed size for source code files in bytes. Files above
    /// this limit will be ignored for performance reasons. Defaults to 1 MiB
    #[bpaf(long("files-max-size"), argument("NUMBER"))]
    pub max_size: Option<NonZeroU64>,

    /// A list of Unix shell style patterns. Biome will ignore files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub ignore: Option<StringSet>,

    /// A list of Unix shell style patterns. Biome will handle only those files/folders that will
    /// match these patterns.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub include: Option<StringSet>,

    /// Tells Biome to not emit diagnostics when handling files that doesn't know
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("files-ignore-unknown"), argument("true|false"), optional)]
    pub ignore_unknown: Option<bool>,
}

impl MergeWith<FilesConfiguration> for FilesConfiguration {
    fn merge_with(&mut self, other: FilesConfiguration) {
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore)
        }
        if let Some(include) = other.include {
            self.include = Some(include)
        }
        if let Some(max_size) = other.max_size {
            self.max_size = Some(max_size)
        }
        if let Some(ignore_unknown) = other.ignore_unknown {
            self.ignore_unknown = Some(ignore_unknown)
        }
    }

    fn merge_with_if_not_default(&mut self, other: FilesConfiguration)
    where
        FilesConfiguration: Default,
    {
        if other != FilesConfiguration::default() {
            self.merge_with(other)
        }
    }
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: Deserialized<Configuration>,
    /// The path of where the `biome.json` file was found. This contains the `biome.json` name.
    pub configuration_file_path: PathBuf,
    /// The base path of where the `biome.json` file was found.
    /// This has to be used to resolve other configuration files.
    pub configuration_directory_path: PathBuf,
}

#[derive(Debug, Default, PartialEq)]
pub enum ConfigurationBasePath {
    /// The default mode, not having a configuration file is not an error.
    #[default]
    None,
    /// The base path provided by the LSP, not having a configuration file is not an error.
    Lsp(PathBuf),
    /// The base path provided by the user, not having a configuration file is an error.
    /// Throws any kind of I/O errors.
    FromUser(PathBuf),
}

impl ConfigurationBasePath {
    const fn is_from_user(&self) -> bool {
        matches!(self, ConfigurationBasePath::FromUser(_))
    }
}

/// Load the configuration for this session of the CLI, merging the content of
/// the `biome.json` file if it exists on disk with common command line options
pub fn load_configuration(
    fs: &DynRef<'_, dyn FileSystem>,
    config_path: ConfigurationBasePath,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = load_config(fs, config_path)?;
    let loaded_configuration = LoadedConfiguration::from(config);
    Ok(loaded_configuration.apply_extends(fs)?)
}

/// Load the configuration from the file system.
///
/// The configuration file will be read from the `file_system`. A [base path](ConfigurationBasePath) should be provided.
///
/// The function will try to traverse upwards the file system until if finds a `biome.json` file, or there
/// aren't directories anymore.
///
/// If a the configuration base path was provided by the user, the function will error. If not, Biome will use
/// its defaults.
fn load_config(
    file_system: &DynRef<'_, dyn FileSystem>,
    base_path: ConfigurationBasePath,
) -> LoadConfig {
    let config_name = file_system.config_name();
    let deprecated_config_name = file_system.deprecated_config_name();
    let working_directory = file_system.working_directory();
    let configuration_directory = match base_path {
        ConfigurationBasePath::Lsp(ref path) | ConfigurationBasePath::FromUser(ref path) => {
            path.clone()
        }
        _ => match working_directory {
            Some(wd) => wd,
            None => PathBuf::new(),
        },
    };
    let should_error = base_path.is_from_user();

    let auto_search_result;
    let result =
        file_system.auto_search(configuration_directory.clone(), config_name, should_error);
    if let Ok(result) = result {
        if result.is_none() {
            auto_search_result = file_system.auto_search(
                configuration_directory.clone(),
                deprecated_config_name,
                should_error,
            )?;
        } else {
            auto_search_result = result;
        }
    } else {
        auto_search_result = file_system.auto_search(
            configuration_directory.clone(),
            deprecated_config_name,
            should_error,
        )?;
    }

    if let Some(auto_search_result) = auto_search_result {
        let AutoSearchResult {
            content,
            directory_path,
            file_path,
        } = auto_search_result;
        let deserialized =
            deserialize_from_json_str::<Configuration>(&content, JsonParserOptions::default());
        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: file_path,
            configuration_directory_path: directory_path,
        }))
    } else {
        Ok(None)
    }
}

/// Creates a new configuration on file system
///
/// ## Errors
///
/// It fails if:
/// - the configuration file already exists
/// - the program doesn't have the write rights
pub fn create_config(
    fs: &mut DynRef<dyn FileSystem>,
    mut configuration: Configuration,
) -> Result<(), WorkspaceError> {
    let path = PathBuf::from(fs.config_name());

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            WorkspaceError::Configuration(ConfigurationDiagnostic::new_already_exists())
        } else {
            WorkspaceError::cant_read_file(format!("{}", path.display()))
        }
    })?;

    // we now check if biome is installed inside `node_modules` and if so, we
    if VERSION == "0.0.0" {
        let schema_path = Path::new("./node_modules/@biomejs/biome/configuration_schema.json");
        let options = OpenOptions::default().read(true);
        if fs.open_with_options(schema_path, options).is_ok() {
            configuration.schema = schema_path.to_str().map(String::from);
        }
    } else {
        configuration.schema = Some(format!(
            "https://biomejs.dev/schemas/{}/schema.json",
            VERSION
        ));
    }

    let contents = serde_json::to_string_pretty(&configuration).map_err(|_| {
        WorkspaceError::Configuration(ConfigurationDiagnostic::new_serialization_error())
    })?;

    let parsed = parse_json(&contents, JsonParserOptions::default());
    let formatted =
        biome_json_formatter::format_node(JsonFormatOptions::default(), &parsed.syntax())?
            .print()
            .expect("valid format document");

    config_file
        .set_content(formatted.as_code().as_bytes())
        .map_err(|_| WorkspaceError::cant_read_file(format!("{}", path.display())))?;

    Ok(())
}

/// Returns the rules applied to a specific [Path], given the [WorkspaceSettings]
pub fn to_analyzer_rules(settings: &WorkspaceSettings, path: &Path) -> AnalyzerRules {
    let linter_settings = &settings.linter;
    let overrides = &settings.override_settings;
    let mut analyzer_rules = AnalyzerRules::default();
    if let Some(rules) = linter_settings.rules.as_ref() {
        push_to_analyzer_rules(rules, metadata(), &mut analyzer_rules);
    }

    overrides.override_analyzer_rules(path, analyzer_rules)
}

#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    pub directory_path: Option<PathBuf>,
    pub file_path: Option<PathBuf>,
    pub configuration: Configuration,
    pub diagnostics: Vec<Error>,
}

impl LoadedConfiguration {
    /// Consumes itself to generate a new [LoadedConfiguration] where the new `configuration`
    /// is the result of its `extends` fields applied from left to right, and the last one element
    /// applied is itself.
    ///
    /// If a configuration can't be resolved from the file system, the operation will fail.
    pub fn apply_extends(
        mut self,
        fs: &DynRef<'_, dyn FileSystem>,
    ) -> Result<Self, WorkspaceError> {
        let deserialized = self.deserialize_extends(fs)?;
        let (configurations, errors): (Vec<_>, Vec<_>) = deserialized
            .into_iter()
            .map(|d| d.consume())
            .map(|(config, diagnostics)| (config.unwrap_or_default(), diagnostics))
            .unzip();

        let extended_configuration = configurations.into_iter().reduce(
            |mut previous_configuration, current_configuration| {
                previous_configuration.merge_with(current_configuration);
                previous_configuration
            },
        );
        let configuration = if let Some(mut extended_configuration) = extended_configuration {
            // Here we want to keep only the values that aren't a default
            extended_configuration.merge_with_if_not_default(self.configuration);
            extended_configuration
        } else {
            self.configuration
        };
        self.diagnostics.extend(
            errors
                .into_iter()
                .flatten()
                .map(|diagnostic| {
                    diagnostic.with_file_path(
                        self.file_path
                            .as_ref()
                            .map(|path| path.display().to_string()),
                    )
                })
                .collect::<Vec<_>>(),
        );

        Ok(Self {
            configuration,
            diagnostics: self.diagnostics,
            file_path: self.file_path,
            directory_path: self.directory_path,
        })
    }

    fn deserialize_extends(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
    ) -> Result<Vec<Deserialized<Configuration>>, WorkspaceError> {
        let Some(extends) = &self.configuration.extends else {
            return Ok(vec![]);
        };

        let directory_path = self
            .directory_path
            .as_ref()
            .cloned()
            .unwrap_or(fs.working_directory().unwrap_or(PathBuf::from("./")));
        let mut deserialized_configurations = vec![];
        for path in extends.iter() {
            let config_path = directory_path.join(path);
            let mut file = fs
                .open_with_options(config_path.as_path(), OpenOptions::default().read(true))
                .map_err(|err| {
                    CantLoadExtendFile::new(config_path.display().to_string(), err.to_string()).with_verbose_advice(
                        markup!{
								"Biome tried to load the configuration file "<Emphasis>{directory_path.display().to_string()}</Emphasis>" using "<Emphasis>{config_path.display().to_string()}</Emphasis>" as base path."
							}
                    )
                })?;
            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|err| {
                CantLoadExtendFile::new(config_path.display().to_string(), err.to_string()).with_verbose_advice(
                    markup!{
							"It's possible that the file was created with a different user/group. Make sure you have the rights to read the file."
						}
                )

            })?;
            let deserialized = deserialize_from_json_str::<Configuration>(
                content.as_str(),
                JsonParserOptions::default(),
            );
            deserialized_configurations.push(deserialized)
        }
        Ok(deserialized_configurations)
    }

    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Path> {
        self.directory_path.as_ref().map(|path| path.as_path())
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_ref().map(|path| path.as_path())
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    /// It extracts diagnostics emitted during the resolution of the configuration file
    pub fn as_diagnostics_iter(&self) -> ConfigurationDiagnosticsIter {
        ConfigurationDiagnosticsIter::new(self.file_path.as_ref(), self.diagnostics.as_slice())
    }
}

pub struct ConfigurationDiagnosticsIter<'a> {
    path: Option<&'a PathBuf>,
    errors: &'a [Error],
    len: usize,
    index: usize,
}

impl<'a> ConfigurationDiagnosticsIter<'a> {
    fn new(path: Option<&'a PathBuf>, errors: &'a [Error]) -> Self {
        Self {
            len: errors.len(),
            index: 0,
            errors,
            path,
        }
    }
}

impl<'a> Iterator for ConfigurationDiagnosticsIter<'a> {
    type Item = (&'a Error, Option<&'a PathBuf>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == self.index {
            return None;
        }

        let item = (self.errors.get(self.index).unwrap(), self.path);

        self.index += 1;
        return Some(item);
    }
}

impl FusedIterator for ConfigurationDiagnosticsIter<'_> {}

impl From<Option<ConfigurationPayload>> for LoadedConfiguration {
    fn from(value: Option<ConfigurationPayload>) -> Self {
        if let Some(value) = value {
            let ConfigurationPayload {
                configuration_directory_path,
                configuration_file_path,
                deserialized,
            } = value;
            let (configuration, diagnostics) = deserialized.consume();
            LoadedConfiguration {
                configuration: configuration.unwrap_or_default(),
                diagnostics: diagnostics
                    .into_iter()
                    .map(|diagnostic| {
                        diagnostic.with_file_path(configuration_file_path.display().to_string())
                    })
                    .collect(),
                directory_path: Some(configuration_directory_path),
                file_path: Some(configuration_file_path),
            }
        } else {
            LoadedConfiguration::default()
        }
    }
}
