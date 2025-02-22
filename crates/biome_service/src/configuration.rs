use crate::settings::Settings;
use crate::WorkspaceError;
use biome_analyze::AnalyzerRules;
use biome_configuration::diagnostics::{CantLoadExtendFile, EditorConfigDiagnostic};
use biome_configuration::{push_to_analyzer_assist, Configuration, VERSION};
use biome_configuration::{
    push_to_analyzer_rules, BiomeDiagnostic, ConfigurationPathHint, ConfigurationPayload,
};
use biome_console::markup;
use biome_css_analyze::METADATA as css_lint_metadata;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::CaminoError;
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_fs::{
    AutoSearchResult, ConfigName, FileSystem, FileSystemDiagnostic, FsErrorKind, OpenOptions,
};
use biome_graphql_analyze::METADATA as graphql_lint_metadata;
use biome_js_analyze::METADATA as js_lint_metadata;
use biome_json_analyze::METADATA as json_lint_metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use camino::{Utf8Path, Utf8PathBuf};
use std::ffi::OsStr;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::path::Path;
use tracing::instrument;

/// Information regarding the configuration that was found.
///
/// This contains the expanded configuration including default values where no
/// configuration was present.
#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    /// If present, the path of the directory where it was found
    pub directory_path: Option<Utf8PathBuf>,
    /// If present, the path of the file where it was found
    pub file_path: Option<Utf8PathBuf>,
    /// The Deserialized configuration
    pub configuration: Configuration,
    /// All diagnostics that were emitted during parsing and deserialization
    pub diagnostics: Vec<Error>,
    /// Whether `biome.json` and `biome.jsonc` were found in the same folder
    pub double_configuration_found: bool,
}

impl LoadedConfiguration {
    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Utf8Path> {
        self.directory_path.as_deref()
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Utf8Path> {
        self.file_path.as_deref()
    }

    /// Whether the are errors emitted. Error are [Severity::Error] or greater.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    /// It returns an iterator over the diagnostics emitted during the resolution of the configuration file
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
        fs: &dyn FileSystem,
    ) -> Result<Self, WorkspaceError> {
        let Some(value) = value else {
            return Ok(LoadedConfiguration::default());
        };

        let ConfigurationPayload {
            external_resolution_base_path,
            configuration_file_path,
            deserialized,
            double_configuration_found,
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
                None => Configuration::default(),
            },
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.with_file_path(configuration_file_path.to_string()))
                .collect(),
            directory_path: configuration_file_path.parent().map(Utf8PathBuf::from),
            file_path: Some(configuration_file_path),
            double_configuration_found,
        })
    }
}

/// Load the partial configuration for this session of the CLI.
#[instrument(level = "debug", skip(fs))]
pub fn load_configuration(
    fs: &dyn FileSystem,
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
/// The configuration file will be read from the `fs`. A [path hint](ConfigurationPathHint) should be provided.
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
#[instrument(level = "debug", skip(fs))]
fn load_config(fs: &dyn FileSystem, base_path: ConfigurationPathHint) -> LoadConfig {
    // This path is used for configuration resolution from external packages.
    let external_resolution_base_path = match &base_path {
        // Path hint from LSP is always the workspace root
        // we use it as the resolution base path.
        ConfigurationPathHint::FromLsp(path) => path.clone(),
        ConfigurationPathHint::FromWorkspace(path) => path.clone(),
        ConfigurationPathHint::FromUser(path) => path.clone(),
        ConfigurationPathHint::None => fs
            .working_directory()
            .map_or(Utf8PathBuf::new(), |working_directory| working_directory),
    };

    // If the configuration path hint is not a file path
    // we'll auto search for the configuration file
    let configuration_directory = match base_path {
        ConfigurationPathHint::FromLsp(path) => path,
        ConfigurationPathHint::FromWorkspace(path) => path,
        ConfigurationPathHint::None => fs.working_directory().unwrap_or_default(),
        ConfigurationPathHint::FromUser(ref config_file_path) => {
            // If the configuration path hint is from user and is a file path, we'll load it directly
            return load_user_config(fs, config_file_path, external_resolution_base_path);
        }
    };
    let biome_json_result = fs.auto_search_file(&configuration_directory, ConfigName::biome_json());

    let biome_jsonc_result =
        fs.auto_search_file(&configuration_directory, ConfigName::biome_jsonc());

    let (auto_search_result, double_configuration_found) =
        match (biome_json_result, biome_jsonc_result) {
            (Some(biome_json_result), Some(_)) => (biome_json_result, true),
            (Some(biome_json_result), None) => (biome_json_result, false),
            (None, Some(biome_jsonc_result)) => (biome_jsonc_result, false),
            (None, None) => {
                return Ok(None);
            }
        };

    // We first search for `biome.json` or `biome.jsonc` files
    let AutoSearchResult {
        content, file_path, ..
    } = auto_search_result;

    let parser_options = match file_path.extension() {
        Some("json") => JsonParserOptions::default(),
        _ => JsonParserOptions::default()
            .with_allow_comments()
            .with_allow_trailing_commas(),
    };

    let deserialized = deserialize_from_json_str::<Configuration>(&content, parser_options, "");

    Ok(Some(ConfigurationPayload {
        deserialized,
        configuration_file_path: file_path,
        external_resolution_base_path,
        double_configuration_found,
    }))
}

fn load_user_config(
    fs: &dyn FileSystem,
    config_file_path: &Utf8Path,
    external_resolution_base_path: Utf8PathBuf,
) -> LoadConfig {
    // If the configuration path hint is from user and is a file path,
    // we'll load it directly
    if fs.path_is_file(config_file_path) {
        let content = fs.read_file_from_path(config_file_path)?;
        let parser_options = match config_file_path.extension() {
            Some("json") => JsonParserOptions::default(),
            Some("jsonc") => JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
            _ => return Err(BiomeDiagnostic::invalid_configuration_file(config_file_path).into()),
        };
        let deserialized = deserialize_from_json_str::<Configuration>(&content, parser_options, "");
        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: config_file_path.to_path_buf(),
            external_resolution_base_path,
            double_configuration_found: false,
        }))
    } else {
        let biome_json_path = config_file_path.join(ConfigName::biome_json());
        let biome_jsonc_path = config_file_path.join(ConfigName::biome_jsonc());
        let biome_json_exists = fs.path_exists(biome_json_path.as_path());
        let biome_jsonc_exists = fs.path_exists(biome_jsonc_path.as_path());

        if !biome_json_exists && !biome_jsonc_exists {
            return Err(BiomeDiagnostic::no_configuration_file_found(config_file_path).into());
        }

        let (config_path, parser_options) = if biome_json_exists {
            (biome_json_path, JsonParserOptions::default())
        } else {
            (
                biome_jsonc_path,
                JsonParserOptions::default()
                    .with_allow_comments()
                    .with_allow_trailing_commas(),
            )
        };

        let content = fs.read_file_from_path(config_path.as_path())?;
        let deserialized = deserialize_from_json_str::<Configuration>(&content, parser_options, "");
        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: config_path.to_path_buf(),
            external_resolution_base_path,
            double_configuration_found: biome_json_exists && biome_jsonc_exists,
        }))
    }
}

pub fn load_editorconfig(
    fs: &dyn FileSystem,
    workspace_root: Utf8PathBuf,
) -> Result<(Option<Configuration>, Vec<EditorConfigDiagnostic>), WorkspaceError> {
    // How .editorconfig is supposed to be resolved: https://editorconfig.org/#file-location
    // We currently don't support the `root` property, so we just search for the file like we do for biome.json
    if let Some(auto_search_result) = fs.auto_search_file(&workspace_root, ".editorconfig") {
        let AutoSearchResult { content, .. } = auto_search_result;
        let editorconfig = biome_configuration::editorconfig::parse_str(&content)?;
        Ok(editorconfig.to_biome())
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
    fs: &dyn FileSystem,
    mut configuration: Configuration,
    emit_jsonc: bool,
) -> Result<(), WorkspaceError> {
    let json_path = Utf8PathBuf::from(ConfigName::biome_json());
    let jsonc_path = Utf8PathBuf::from(ConfigName::biome_jsonc());

    if fs.path_exists(&json_path) || fs.path_exists(&jsonc_path) {
        return Err(BiomeDiagnostic::new_already_exists().into());
    }

    let path = if emit_jsonc { jsonc_path } else { json_path };

    let options = OpenOptions::default().write(true).create_new(true);

    let mut config_file = fs.open_with_options(&path, options).map_err(|err| {
        if err.kind() == ErrorKind::AlreadyExists {
            BiomeDiagnostic::new_already_exists().into()
        } else {
            WorkspaceError::cant_read_file(path.to_string())
        }
    })?;

    // we now check if biome is installed inside `node_modules` and if so, we
    if VERSION == "0.0.0" {
        let schema_path = Utf8Path::new("./node_modules/@biomejs/biome/configuration_schema.json");
        let options = OpenOptions::default().read(true);
        if fs.open_with_options(schema_path, options).is_ok() {
            configuration.schema = Some(Box::from(schema_path.as_str()));
        }
    } else {
        configuration.schema =
            Some(format!("https://biomejs.dev/schemas/{VERSION}/schema.json").into());
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
        .map_err(|_| WorkspaceError::cant_read_file(format!("{}", path)))?;

    Ok(())
}

/// Returns the rules applied to a specific [Path], given the [Settings]
pub fn to_analyzer_rules(settings: &Settings, path: &Utf8Path) -> AnalyzerRules {
    let mut analyzer_rules = AnalyzerRules::default();
    if let Some(rules) = settings.linter.rules.as_ref() {
        push_to_analyzer_rules(rules, js_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, css_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, json_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, graphql_lint_metadata.deref(), &mut analyzer_rules);
    }
    if let Some(rules) = settings.assist.actions.as_ref() {
        push_to_analyzer_assist(rules, js_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_assist(rules, css_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_assist(rules, json_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_assist(rules, graphql_lint_metadata.deref(), &mut analyzer_rules);
    }
    let overrides = &settings.override_settings;
    overrides.override_analyzer_rules(path, analyzer_rules)
}

pub trait ConfigurationExt {
    fn apply_extends(
        &mut self,
        fs: &dyn FileSystem,
        file_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
        diagnostics: &mut Vec<Error>,
    ) -> Result<(), WorkspaceError>;

    fn deserialize_extends(
        &mut self,
        fs: &dyn FileSystem,
        relative_resolution_base_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
    ) -> Result<Vec<Deserialized<Configuration>>, WorkspaceError>;

    fn migrate_deprecated_fields(&mut self);

    fn retrieve_gitignore_matches(
        &self,
        fs: &dyn FileSystem,
        vcs_base_path: Option<&Utf8Path>,
    ) -> Result<(Option<Utf8PathBuf>, Vec<String>), WorkspaceError>;
}

impl ConfigurationExt for Configuration {
    /// Mutates the configuration so that any fields that have not been configured explicitly are
    /// filled in with their values from configs listed in the `extends` field.
    ///
    /// The `extends` configs are applied from left to right.
    ///
    /// If a configuration can't be resolved from the file system, the operation will fail.
    fn apply_extends(
        &mut self,
        fs: &dyn FileSystem,
        file_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
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
                .map(|diagnostic| diagnostic.with_file_path(file_path.to_string()))
                .collect::<Vec<_>>(),
        );

        Ok(())
    }

    /// It attempts to deserialize all the configuration files that were specified in the `extends` property
    fn deserialize_extends(
        &mut self,
        fs: &dyn FileSystem,
        relative_resolution_base_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
    ) -> Result<Vec<Deserialized<Configuration>>, WorkspaceError> {
        let Some(extends) = &self.extends else {
            return Ok(Vec::new());
        };

        let mut deserialized_configurations = vec![];
        for extend_entry in extends.iter() {
            let extend_entry_as_path = Path::new(extend_entry.as_ref());

            let extend_configuration_file_path = if extend_entry_as_path.starts_with(".")
                // TODO: Remove extension in Biome 2.0
                || matches!(
                    extend_entry_as_path.extension().map(OsStr::as_encoded_bytes),
                    Some(b"json" | b"jsonc")
                ) {
                relative_resolution_base_path.join(extend_entry.as_ref())
            } else {
                Utf8PathBuf::try_from(
                    fs.resolve_configuration(extend_entry.as_ref(), external_resolution_base_path)
                        .map_err(|error| {
                            BiomeDiagnostic::cant_resolve(
                                external_resolution_base_path.to_string(),
                                error,
                            )
                        })?
                        .into_path_buf(),
                )
                .map_err(|err| FileSystemDiagnostic {
                    path: external_resolution_base_path.to_string(),
                    severity: Severity::Error,
                    error_kind: FsErrorKind::CantReadFile,
                    source: Some(Error::from(CaminoError::from(err))),
                })?
            };

            let mut file = fs
                .open_with_options(
                    extend_configuration_file_path.as_path(),
                    OpenOptions::default().read(true),
                )
                .map_err(|err| {
                    CantLoadExtendFile::new(
                        extend_configuration_file_path.to_string(),
                        err.to_string(),
                    )
                    .with_verbose_advice(markup! {
                        "Biome tried to load the configuration file \""<Emphasis>{
                            extend_configuration_file_path.to_string()
                        }</Emphasis>"\" in \"extends\" using \""<Emphasis>{
                            external_resolution_base_path.to_string()
                        }</Emphasis>"\" as the base path."
                    })
                })?;

            let mut content = String::new();
            file.read_to_string(&mut content).map_err(|err| {
                CantLoadExtendFile::new(extend_configuration_file_path.to_string(), err.to_string()).with_verbose_advice(
                    markup!{
                        "It's possible that the file was created with a different user/group. Make sure you have the rights to read the file."
                    }
                )

            })?;
            let deserialized = deserialize_from_json_str::<Configuration>(
                content.as_str(),
                match extend_configuration_file_path.extension() {
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
    fn migrate_deprecated_fields(&mut self) {}

    /// This function checks if the VCS integration is enabled, and if so, it will attempts to resolve the
    /// VCS root directory and the `.gitignore` file.
    ///
    /// ## Returns
    ///
    /// A tuple with VCS root folder and the contents of the `.gitignore` file
    fn retrieve_gitignore_matches(
        &self,
        fs: &dyn FileSystem,
        vcs_base_path: Option<&Utf8Path>,
    ) -> Result<(Option<Utf8PathBuf>, Vec<String>), WorkspaceError> {
        let Some(vcs) = &self.vcs else {
            return Ok((None, vec![]));
        };
        if vcs.is_enabled() {
            let vcs_base_path = match (vcs_base_path, &vcs.root) {
                (Some(vcs_base_path), Some(root)) => vcs_base_path.join(root),
                (None, Some(root)) => Utf8PathBuf::from(root),
                (Some(vcs_base_path), None) => Utf8PathBuf::from(vcs_base_path),
                (None, None) => return Err(WorkspaceError::vcs_disabled()),
            };
            if let Some(client_kind) = &vcs.client_kind {
                if vcs.should_use_ignore_file() {
                    let result = fs.auto_search_file(&vcs_base_path, client_kind.ignore_file());

                    if let Some(result) = result {
                        return Ok((
                            result.file_path.parent().map(Utf8PathBuf::from),
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

#[cfg(test)]
mod test {
    use crate::configuration::load_configuration;
    use biome_configuration::ConfigurationPathHint;
    use biome_fs::MemoryFileSystem;
    use camino::Utf8PathBuf;

    #[test]
    fn should_not_load_a_configuration_yml() {
        let mut fs = MemoryFileSystem::default();
        fs.insert(Utf8PathBuf::from("biome.yml"), "content".to_string());
        let path_hint = ConfigurationPathHint::FromUser(Utf8PathBuf::from("biome.yml"));

        let result = load_configuration(&fs, path_hint);

        assert!(result.is_err());
    }

    #[test]
    fn should_print_a_warning_for_double_configuration() {
        let mut fs = MemoryFileSystem::default();
        fs.insert(Utf8PathBuf::from("config/biome.json"), "{}".to_string());
        fs.insert(Utf8PathBuf::from("config/biome.jsonc"), "{}".to_string());
        let path_hint = ConfigurationPathHint::FromUser(Utf8PathBuf::from("config"));

        let result = load_configuration(&fs, path_hint);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.double_configuration_found);
    }
}
