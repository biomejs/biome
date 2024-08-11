use crate::matcher::Pattern;
use crate::settings::Settings;
use crate::{DynRef, WorkspaceError};
use biome_analyze::AnalyzerRules;
use biome_configuration::diagnostics::{CantLoadExtendFile, EditorConfigDiagnostic};
use biome_configuration::VERSION;
use biome_configuration::{
    push_to_analyzer_rules, BiomeDiagnostic, ConfigurationPathHint, ConfigurationPayload,
    PartialConfiguration,
};
use biome_console::markup;
use biome_css_analyze::METADATA as css_lint_metadata;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_fs::{AutoSearchResult, ConfigName, FileSystem, OpenOptions};
use biome_graphql_analyze::METADATA as graphql_lint_metadata;
use biome_js_analyze::METADATA as js_lint_metadata;
use biome_json_analyze::METADATA as json_lint_metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::path::{Path, PathBuf};

/// Information regarding the configuration that was found.
///
/// This contains the expanded configuration including default values where no
/// configuration was present.
#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    /// If present, the path of the directory where it was found
    pub directory_path: Option<PathBuf>,
    /// If present, the path of the file where it was found
    pub file_path: Option<PathBuf>,
    /// The Deserialized configuration
    pub configuration: PartialConfiguration,
    /// All diagnostics that were emitted during parsing and deserialization
    pub diagnostics: Vec<Error>,
}

impl LoadedConfiguration {
    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Path> {
        self.directory_path.as_deref()
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Path> {
        self.file_path.as_deref()
    }

    /// Whether the are errors emitted. Error are [Severity::Error] or greater.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    /// It return an iterator over the diagnostics emitted during the resolution of the configuration file
    pub fn as_diagnostics_iter(&self) -> ConfigurationDiagnosticsIter {
        ConfigurationDiagnosticsIter::new(self.diagnostics.as_slice())
    }
}

pub struct ConfigurationDiagnosticsIter<'a> {
    errors: &'a [Error],
    len: usize,
    index: usize,
}

impl<'a> ConfigurationDiagnosticsIter<'a> {
    fn new(errors: &'a [Error]) -> Self {
        Self {
            len: errors.len(),
            index: 0,
            errors,
        }
    }
}

impl<'a> Iterator for ConfigurationDiagnosticsIter<'a> {
    type Item = &'a Error;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len == self.index {
            return None;
        }

        let item = self.errors.get(self.index);
        self.index += 1;
        item
    }
}

impl FusedIterator for ConfigurationDiagnosticsIter<'_> {}

impl LoadedConfiguration {
    fn try_from_payload(
        value: Option<ConfigurationPayload>,
        fs: &DynRef<'_, dyn FileSystem>,
    ) -> Result<Self, WorkspaceError> {
        let Some(value) = value else {
            return Ok(LoadedConfiguration::default());
        };

        let ConfigurationPayload {
            external_resolution_base_path,
            configuration_file_path,
            deserialized,
        } = value;
        let (partial_configuration, mut diagnostics) = deserialized.consume();

        Ok(Self {
            configuration: match partial_configuration {
                Some(mut partial_configuration) => {
                    partial_configuration.apply_extends(
                        fs,
                        &configuration_file_path,
                        &external_resolution_base_path,
                        &mut diagnostics,
                    )?;
                    partial_configuration.migrate_deprecated_fields();
                    partial_configuration
                }
                None => PartialConfiguration::default(),
            },
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| {
                    diagnostic.with_file_path(configuration_file_path.display().to_string())
                })
                .collect(),
            directory_path: configuration_file_path.parent().map(PathBuf::from),
            file_path: Some(configuration_file_path),
        })
    }
}

/// Load the partial configuration for this session of the CLI.
pub fn load_configuration(
    fs: &DynRef<'_, dyn FileSystem>,
    config_path: ConfigurationPathHint,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = load_config(fs, config_path)?;
    LoadedConfiguration::try_from_payload(config, fs)
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

/// Load the configuration from the file system.
///
/// The configuration file will be read from the `file_system`. A [path hint](ConfigurationPathHint) should be provided.
///
/// - If the path hint is a path to a file that is provided by the user, the function will try to load that file or error.
///     The name doesn't have to be `biome.json` or `biome.jsonc`. And if it doesn't end with `.json`, Biome will try to
///     deserialize it as a `.jsonc` file.
///
/// - If the path hint is a path to a directory which is provided by the user, the function will try to find a `biome.json`
///     or `biome.jsonc` file in order in that directory. And If it cannot find one, it will error.
///
/// - Otherwise, the function will try to traverse upwards the file system until it finds a `biome.json` or `biome.jsonc`
///     file, or there aren't directories anymore. In this case, the function will not error but return an `Ok(None)`, which
///     means Biome will use the default configuration.
fn load_config(
    file_system: &DynRef<'_, dyn FileSystem>,
    base_path: ConfigurationPathHint,
) -> LoadConfig {
    // This path is used for configuration resolution from external packages.
    let external_resolution_base_path = match base_path {
        // Path hint from LSP is always the workspace root
        // we use it as the resolution base path.
        ConfigurationPathHint::FromLsp(ref path) => path.clone(),
        ConfigurationPathHint::FromWorkspace(ref path) => path.clone(),
        // Path hint from user means the command is invoked from the CLI
        // So we use the working directory (CWD) as the resolution base path
        ConfigurationPathHint::FromUser(_) | ConfigurationPathHint::None => file_system
            .working_directory()
            .map_or(PathBuf::new(), |working_directory| working_directory),
    };

    // If the configuration path hint is from user and is a file path,
    // we'll load it directly
    if let ConfigurationPathHint::FromUser(ref configuration_file_path) = base_path {
        if file_system.path_is_file(configuration_file_path) {
            let content = file_system.read_file_from_path(configuration_file_path)?;
            let parser_options = match configuration_file_path.extension().and_then(OsStr::to_str) {
                Some("json") => JsonParserOptions::default(),
                _ => JsonParserOptions::default()
                    .with_allow_comments()
                    .with_allow_trailing_commas(),
            };
            let deserialized =
                deserialize_from_json_str::<PartialConfiguration>(&content, parser_options, "");
            return Ok(Some(ConfigurationPayload {
                deserialized,
                configuration_file_path: PathBuf::from(configuration_file_path),
                external_resolution_base_path,
            }));
        }
    }

    // If the configuration path hint is not a file path
    // we'll auto search for the configuration file
    let should_error = base_path.is_from_user();
    let configuration_directory = match base_path {
        ConfigurationPathHint::FromLsp(path) => path,
        ConfigurationPathHint::FromUser(path) => path,
        ConfigurationPathHint::FromWorkspace(path) => path,
        ConfigurationPathHint::None => file_system.working_directory().unwrap_or_default(),
    };

    // We first search for `biome.json` or `biome.jsonc` files
    if let Some(auto_search_result) = match file_system.auto_search(
        &configuration_directory,
        ConfigName::file_names().as_slice(),
        should_error,
    ) {
        Ok(Some(auto_search_result)) => Some(auto_search_result),
        // We then search for the deprecated `rome.json` file
        // if neither `biome.json` nor `biome.jsonc` is found
        // TODO: The following arms should be removed in v2.0.0
        Ok(None) => file_system.auto_search(
            &configuration_directory,
            [file_system.deprecated_config_name()].as_slice(),
            should_error,
        )?,
        Err(error) => file_system
            .auto_search(
                &configuration_directory,
                [file_system.deprecated_config_name()].as_slice(),
                should_error,
            )
            // Map the error so users won't see error messages
            // that contains `rome.json`
            .map_err(|_| error)?,
    } {
        let AutoSearchResult { content, file_path } = auto_search_result;

        let parser_options = match file_path.extension().and_then(OsStr::to_str) {
            Some("json") => JsonParserOptions::default(),
            _ => JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
        };

        let deserialized =
            deserialize_from_json_str::<PartialConfiguration>(&content, parser_options, "");

        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: file_path,
            external_resolution_base_path,
        }))
    } else {
        Ok(None)
    }
}

pub fn load_editorconfig(
    file_system: &DynRef<'_, dyn FileSystem>,
    workspace_root: PathBuf,
) -> Result<(Option<PartialConfiguration>, Vec<EditorConfigDiagnostic>), WorkspaceError> {
    // How .editorconfig is supposed to be resolved: https://editorconfig.org/#file-location
    // We currently don't support the `root` property, so we just search for the file like we do for biome.json
    if let Some(auto_search_result) =
        match file_system.auto_search(&workspace_root, [".editorconfig"].as_slice(), false) {
            Ok(result) => result,
            Err(error) => return Err(WorkspaceError::from(error)),
        }
    {
        let AutoSearchResult {
            content,
            file_path: path,
        } = auto_search_result;
        let editorconfig = biome_configuration::editorconfig::parse_str(&content)?;
        let config = editorconfig.to_biome();

        // test the patterns to see if they are parsable so we can emit a better diagnostic
        if let Some(overrides) = config.0.as_ref().and_then(|c| c.overrides.as_ref()) {
            for override_pattern in &overrides.0 {
                if let Some(pattern_set) = &override_pattern.include {
                    for pattern in pattern_set.iter() {
                        if let Err(err) = Pattern::new(pattern) {
                            return Err(BiomeDiagnostic::new_invalid_ignore_pattern_with_path(
                                pattern,
                                err.to_string(),
                                path.to_str(),
                            )
                            .into());
                        }
                    }
                }
            }
        }

        Ok(config)
    } else {
        Ok((None, vec![]))
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
    mut configuration: PartialConfiguration,
    emit_jsonc: bool,
) -> Result<(), WorkspaceError> {
    let json_path = PathBuf::from(ConfigName::biome_json());
    let jsonc_path = PathBuf::from(ConfigName::biome_jsonc());

    if fs.path_exists(&json_path) || fs.path_exists(&jsonc_path) {
        return Err(BiomeDiagnostic::new_already_exists().into());
    }

    let path = if emit_jsonc { jsonc_path } else { json_path };

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            BiomeDiagnostic::new_already_exists().into()
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
        configuration.schema = Some(format!("https://biomejs.dev/schemas/{VERSION}/schema.json"));
    }

    let contents = serde_json::to_string_pretty(&configuration)
        .map_err(|_| BiomeDiagnostic::new_serialization_error())?;

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

/// Returns the rules applied to a specific [Path], given the [Settings]
pub fn to_analyzer_rules(settings: &Settings, path: &Path) -> AnalyzerRules {
    let linter_settings = &settings.linter;
    let overrides = &settings.override_settings;
    let mut analyzer_rules = AnalyzerRules::default();
    if let Some(rules) = linter_settings.rules.as_ref() {
        push_to_analyzer_rules(rules, js_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, css_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, json_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, graphql_lint_metadata.deref(), &mut analyzer_rules);
    }

    overrides.override_analyzer_rules(path, analyzer_rules)
}

pub trait PartialConfigurationExt {
    fn apply_extends(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
        file_path: &Path,
        external_resolution_base_path: &Path,
        diagnostics: &mut Vec<Error>,
    ) -> Result<(), WorkspaceError>;

    fn deserialize_extends(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
        relative_resolution_base_path: &Path,
        external_resolution_base_path: &Path,
    ) -> Result<Vec<Deserialized<PartialConfiguration>>, WorkspaceError>;

    fn migrate_deprecated_fields(&mut self);

    fn retrieve_gitignore_matches(
        &self,
        file_system: &DynRef<'_, dyn FileSystem>,
        vcs_base_path: Option<&Path>,
    ) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError>;
}

impl PartialConfigurationExt for PartialConfiguration {
    /// Mutates the configuration so that any fields that have not been configured explicitly are
    /// filled in with their values from configs listed in the `extends` field.
    ///
    /// The `extends` configs are applied from left to right.
    ///
    /// If a configuration can't be resolved from the file system, the operation will fail.
    fn apply_extends(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
        file_path: &Path,
        external_resolution_base_path: &Path,
        diagnostics: &mut Vec<Error>,
    ) -> Result<(), WorkspaceError> {
        let deserialized = self.deserialize_extends(
            fs,
            file_path.parent().expect("file path should have a parent"),
            external_resolution_base_path,
        )?;
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
        if let Some(mut extended_configuration) = extended_configuration {
            // We swap them to avoid having to clone `self.configuration` to merge it.
            std::mem::swap(self, &mut extended_configuration);
            self.merge_with(extended_configuration)
        }

        diagnostics.extend(
            errors
                .into_iter()
                .flatten()
                .map(|diagnostic| diagnostic.with_file_path(file_path.display().to_string()))
                .collect::<Vec<_>>(),
        );

        Ok(())
    }

    /// It attempts to deserialize all the configuration files that were specified in the `extends` property
    fn deserialize_extends(
        &mut self,
        fs: &DynRef<'_, dyn FileSystem>,
        relative_resolution_base_path: &Path,
        external_resolution_base_path: &Path,
    ) -> Result<Vec<Deserialized<PartialConfiguration>>, WorkspaceError> {
        let Some(extends) = &self.extends else {
            return Ok(Vec::new());
        };

        let mut deserialized_configurations = vec![];
        for extend_entry in extends.iter() {
            let extend_entry_as_path = Path::new(extend_entry);

            let extend_configuration_file_path = if extend_entry_as_path.starts_with(".")
                // TODO: Remove extension in Biome 2.0
                || matches!(
                    extend_entry_as_path.extension().and_then(OsStr::to_str),
                    Some("json" | "jsonc")
                ) {
                relative_resolution_base_path.join(extend_entry)
            } else {
                fs.resolve_configuration(extend_entry.as_str(), external_resolution_base_path)
                    .map_err(|error| {
                        BiomeDiagnostic::cant_resolve(
                            external_resolution_base_path.display().to_string(),
                            error,
                        )
                    })?
                    .into_path_buf()
            };

            let mut file = fs
                .open_with_options(
                    extend_configuration_file_path.as_path(),
                    OpenOptions::default().read(true),
                )
                .map_err(|err| {
                    CantLoadExtendFile::new(
                        extend_configuration_file_path.display().to_string(),
                        err.to_string(),
                    )
                    .with_verbose_advice(markup! {
                        "Biome tried to load the configuration file \""<Emphasis>{
                            extend_configuration_file_path.display().to_string()
                        }</Emphasis>"\" in \"extends\" using \""<Emphasis>{
                            external_resolution_base_path.display().to_string()
                        }</Emphasis>"\" as the base path."
                    })
                })?;

            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|err| {
                CantLoadExtendFile::new(extend_configuration_file_path.display().to_string(), err.to_string()).with_verbose_advice(
                    markup!{
                        "It's possible that the file was created with a different user/group. Make sure you have the rights to read the file."
                    }
                )

            })?;
            let deserialized = deserialize_from_json_str::<PartialConfiguration>(
                content.as_str(),
                match extend_configuration_file_path
                    .extension()
                    .and_then(OsStr::to_str)
                {
                    Some("json") => JsonParserOptions::default(),
                    _ => JsonParserOptions::default()
                        .with_allow_comments()
                        .with_allow_trailing_commas(),
                },
                "",
            );
            deserialized_configurations.push(deserialized)
        }
        Ok(deserialized_configurations)
    }

    /// Checks for the presence of deprecated fields and updates the
    /// configuration to apply them to the new schema.
    fn migrate_deprecated_fields(&mut self) {
        // TODO: remove in biome 2.0
        if let Some(formatter) = self.formatter.as_mut() {
            if formatter.indent_size.is_some() && formatter.indent_width.is_none() {
                formatter.indent_width = formatter.indent_size;
            }
        }

        // TODO: remove in biome 2.0
        if let Some(formatter) = self
            .javascript
            .as_mut()
            .and_then(|js| js.formatter.as_mut())
        {
            if formatter.indent_size.is_some() && formatter.indent_width.is_none() {
                formatter.indent_width = formatter.indent_size;
            }

            if formatter.trailing_comma.is_some() && formatter.trailing_commas.is_none() {
                formatter.trailing_commas = formatter.trailing_comma;
            }
        }

        // TODO: remove in biome 2.0
        if let Some(formatter) = self.json.as_mut().and_then(|json| json.formatter.as_mut()) {
            if formatter.indent_size.is_some() && formatter.indent_width.is_none() {
                formatter.indent_width = formatter.indent_size;
            }
        }
    }

    /// This function checks if the VCS integration is enabled, and if so, it will attempts to resolve the
    /// VCS root directory and the `.gitignore` file.
    ///
    /// ## Returns
    ///
    /// A tuple with VCS root folder and the contents of the `.gitignore` file
    fn retrieve_gitignore_matches(
        &self,
        file_system: &DynRef<'_, dyn FileSystem>,
        vcs_base_path: Option<&Path>,
    ) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError> {
        let Some(vcs) = &self.vcs else {
            return Ok((None, vec![]));
        };
        if vcs.is_enabled() {
            let vcs_base_path = match (vcs_base_path, &vcs.root) {
                (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
                (None, Some(root)) => PathBuf::from(root),
                (Some(vcs_base_path), None) => PathBuf::from(vcs_base_path),
                (None, None) => return Err(WorkspaceError::vcs_disabled()),
            };
            if let Some(client_kind) = &vcs.client_kind {
                if !vcs.ignore_file_disabled() {
                    let result = file_system
                        .auto_search(&vcs_base_path, &[client_kind.ignore_file()], false)
                        .map_err(WorkspaceError::from)?;

                    if let Some(result) = result {
                        return Ok((
                            result.file_path.parent().map(PathBuf::from),
                            result
                                .content
                                .lines()
                                .map(String::from)
                                .collect::<Vec<String>>(),
                        ));
                    }
                }
            }
        }
        Ok((None, vec![]))
    }
}
