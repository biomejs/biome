use crate::WorkspaceError;
use crate::settings::Settings;
use crate::workspace::ScanKind;
use biome_analyze::{
    AnalyzerRules, Queryable, RegistryVisitor, Rule, RuleDomain, RuleFilter, RuleGroup,
};
use biome_configuration::analyzer::{AnalyzerSelector, RuleDomainValue};
use biome_configuration::diagnostics::{
    CantLoadExtendFile, CantResolve, EditorConfigDiagnostic, ParseFailedDiagnostic,
};
use biome_configuration::editorconfig::EditorConfig;
use biome_configuration::{
    BiomeDiagnostic, ConfigurationPathHint, ConfigurationPayload, push_to_analyzer_rules,
};
use biome_configuration::{Configuration, VERSION, push_to_analyzer_assist};
use biome_console::markup;
use biome_css_analyze::METADATA as css_lint_metadata;
use biome_css_syntax::CssLanguage;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_fs::{AutoSearchResult, ConfigName, FileSystem, OpenOptions};
use biome_graphql_analyze::METADATA as graphql_lint_metadata;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_analyze::METADATA as js_lint_metadata;
use biome_js_syntax::JsLanguage;
use biome_json_analyze::METADATA as json_lint_metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::JsonLanguage;
use biome_resolver::{FsWithResolverProxy, ResolveOptions, resolve};
use biome_rowan::Language;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::FxHashSet;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::path::Path;
use std::str::FromStr;
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
}

impl LoadedConfiguration {
    /// It consumes the payload, applies and extends and returns the final, extended configuration.
    pub fn try_from_payload(
        value: Option<ConfigurationPayload>,
        fs: &dyn FsWithResolverProxy,
    ) -> Result<Self, WorkspaceError> {
        let Some(value) = value else {
            return Ok(Self::default());
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
                None => Configuration::default(),
            },
            diagnostics: diagnostics
                .into_iter()
                .map(|diagnostic| diagnostic.with_file_path(configuration_file_path.to_string()))
                .collect(),
            directory_path: configuration_file_path.parent().map(Utf8PathBuf::from),
            file_path: Some(configuration_file_path),
        })
    }

    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Utf8Path> {
        self.directory_path.as_deref()
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Utf8Path> {
        self.file_path.as_deref()
    }

    /// Whether they are errors emitted. Error are [Severity::Error] or greater.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    /// It returns an iterator over the diagnostics emitted during the resolution of the configuration file
    pub fn as_diagnostics_iter(&self) -> ConfigurationDiagnosticsIter<'_> {
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

/// Load the partial configuration for this session.
#[instrument(level = "debug", skip(fs))]
pub fn load_configuration(
    fs: &dyn FsWithResolverProxy,
    config_path: ConfigurationPathHint,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = read_config(fs, config_path, true)?;
    LoadedConfiguration::try_from_payload(config, fs)
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

/// Loads the configuration from the file system.
///
/// The configuration file will be read from the `fs`.
///
/// A [`path_hint`](ConfigurationPathHint) should be provided.
///
/// - If the path hint is a path to a file that is provided by the user, the
///     function will try to load that file or error. The name doesn't have to
///     be `biome.json` or `biome.jsonc`. And if it doesn't end with `.json`,
///     Biome will try to deserialize it as a `.jsonc` file.
///
/// - If the path hint is a path to a directory which is provided by the user,
///     the function will try to find a `biome.json` or `biome.jsonc` file in
///     order in that directory. And if it cannot find one, it will error.
///
/// - Otherwise, the function will try to traverse upwards through the file
///     system until it finds a `biome.json` or `biome.jsonc` file, or there
///     aren't directories anymore. In this case, the function will not error
///     but return an `Ok(None)`, which means Biome will use the default
///     configuration.
///
/// If `seek_root` is `true`, the function will stop at the first
/// configuration file with `"root": true`. Otherwise, any configuration file
/// will do.
#[instrument(level = "debug", skip(fs))]
pub fn read_config(
    fs: &dyn FileSystem,
    path_hint: ConfigurationPathHint,
    seek_root: bool,
) -> LoadConfig {
    // This path is used for configuration resolution from external packages.
    let external_resolution_base_path = match &path_hint {
        // Path hint from LSP is always the workspace root
        // we use it as the resolution base path.
        ConfigurationPathHint::FromLsp(path) => path.clone(),
        ConfigurationPathHint::FromWorkspace(path) => path.clone(),
        ConfigurationPathHint::FromUser(path) => path.clone(),
        ConfigurationPathHint::None => fs.working_directory().unwrap_or_default(),
    };

    // If the configuration path hint is not a file path
    // we'll auto search for the configuration file
    let configuration_directory = match path_hint {
        ConfigurationPathHint::FromLsp(path) => path,
        ConfigurationPathHint::FromWorkspace(path) => path,
        ConfigurationPathHint::FromUser(ref config_file_path) => {
            // If the configuration path hint is from the user, we'll load it
            // directly.
            return load_user_config(fs, config_file_path, external_resolution_base_path);
        }
        ConfigurationPathHint::None => fs.working_directory().unwrap_or_default(),
    };

    // We search for the first non-root `biome.json` or `biome.jsonc` files:
    let mut deserialized = None;
    let mut predicate = |file_path: &Utf8Path, content: &str| -> bool {
        let parser_options = match file_path.extension() {
            Some("json") => JsonParserOptions::default(),
            _ => JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas(),
        };

        let deserialized_content =
            deserialize_from_json_str::<Configuration>(content, parser_options, "");
        let is_found = deserialized_content
            .deserialized
            .as_ref()
            .is_some_and(|config| if seek_root { config.is_root() } else { true });
        if is_found {
            deserialized = Some(deserialized_content);
        }
        is_found
    };

    let Some(auto_search_result) = fs.auto_search_files_with_predicate(
        &configuration_directory,
        &[ConfigName::biome_json(), ConfigName::biome_jsonc()],
        &mut predicate,
    ) else {
        return Ok(None);
    };

    Ok(Some(ConfigurationPayload {
        // Unwrapping is safe because the predicate in the search above would
        // only return `true` if it assigned `Some` value:
        deserialized: deserialized.unwrap(),
        configuration_file_path: auto_search_result.file_path,
        external_resolution_base_path,
    }))
}

fn load_user_config(
    fs: &dyn FileSystem,
    config_file_path: &Utf8Path,
    external_resolution_base_path: Utf8PathBuf,
) -> LoadConfig {
    // If the configuration path hint is a file path, we'll load it directly.
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
        if deserialized
            .deserialized
            .as_ref()
            .is_some_and(|config| config.root.is_some_and(|root| !root.value()))
        {
            return Err(BiomeDiagnostic::non_root_configuration(config_file_path).into());
        }

        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: config_path.to_path_buf(),
            external_resolution_base_path,
        }))
    }
}

/// judge if path a is parent path for path b.
fn is_parent_of(a: Utf8PathBuf, b: Utf8PathBuf) -> bool {
    if a == b {
        return false;
    }

    if let Ok(relative_path) = b.strip_prefix(a) {
        !relative_path.has_root()
    } else {
        false
    }
}

pub fn load_editorconfig(
    fs: &dyn FileSystem,
    workspace_root: Utf8PathBuf,
    config_path: Option<Utf8PathBuf>,
) -> Result<(Option<Configuration>, Vec<EditorConfigDiagnostic>), WorkspaceError> {
    // How .editorconfig is supposed to be resolved: https://editorconfig.org/#file-location
    // We currently don't support the `root` property, so we just search for the file like we do for biome.json.
    // And we make some judge for the case when `biome.json` and `.editorconfig` both exists.
    // If we found a `.editorconfig` directory higher than `biome.json`, we'll don't use it.
    if let Some(auto_search_result) = fs.auto_search_files(&workspace_root, &[".editorconfig"]) {
        let AutoSearchResult {
            content,
            file_path,
            directory_path,
        } = auto_search_result;
        let editorconfig = EditorConfig::from_str(&content).map_err(|err| {
            EditorConfigDiagnostic::ParseFailed(ParseFailedDiagnostic {
                kind: err.kind,
                path: file_path.into_string(),
                source_code: content,
                span: err.span,
            })
        })?;
        if let Some(config_path) = config_path {
            // if `.edirotconfig` is higher than `biome.json`
            if is_parent_of(directory_path, config_path) {
                Ok((None, vec![]))
            } else {
                Ok(editorconfig.to_biome())
            }
        } else {
            // If we don't find `biome.json`, we'll use `.editorconfig`
            Ok(editorconfig.to_biome())
        }
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
            configuration.schema = Some(schema_path.to_string().into());
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
        .map_err(|_| WorkspaceError::cant_read_file(format!("{path}")))?;

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
        fs: &dyn FsWithResolverProxy,
        file_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
        diagnostics: &mut Vec<Error>,
    ) -> Result<(), WorkspaceError>;

    fn deserialize_extends(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        relative_resolution_base_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
    ) -> Result<Vec<Deserialized<Configuration>>, WorkspaceError>;

    fn migrate_deprecated_fields(&mut self);
}

impl ConfigurationExt for Configuration {
    /// Mutates the configuration so that any fields that have not been
    /// configured explicitly are filled in with their values from configs
    /// listed in the `extends` field.
    ///
    /// The `extends` configs are applied from left to right.
    ///
    /// If a configuration can't be resolved from the file system, the operation
    /// will fail.
    ///
    /// `file_path` is the path to the configuration file and is used for
    /// resolving relative paths in the `extends` field.
    ///
    /// `external_resolution_base_path` is used for resolving non-relative
    /// `extends` entries.
    fn apply_extends(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        file_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
        diagnostics: &mut Vec<Error>,
    ) -> Result<(), WorkspaceError> {
        let deserialized = self.deserialize_extends(
            fs,
            file_path.parent().expect("file path should have a parent"),
            external_resolution_base_path,
        )?;
        let (configurations, errors): (Vec<_>, Vec<_>) =
            deserialized.into_iter().map(Deserialized::consume).unzip();

        let extended_configuration = configurations.into_iter().flatten().reduce(
            |mut previous_configuration, current_configuration| {
                previous_configuration.merge_with(current_configuration);
                previous_configuration
            },
        );
        if let Some(mut extended_configuration) = extended_configuration {
            // Make sure our root value is set explicitly, so it cannot be set
            // by configs we extend.
            self.root = Some(self.is_root().into());

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

    /// Deserializes all the configuration files that were specified in the
    /// `extends` field.
    fn deserialize_extends(
        &mut self,
        fs: &dyn FsWithResolverProxy,
        relative_resolution_base_path: &Utf8Path,
        external_resolution_base_path: &Utf8Path,
    ) -> Result<Vec<Deserialized<Configuration>>, WorkspaceError> {
        let Some(extends) = &self.extends else {
            return Ok(Vec::new());
        };

        let mut deserialized_configurations = vec![];
        if let Some(extends) = extends.as_list() {
            for extend_entry in extends.iter() {
                let extend_entry_as_path = Path::new(extend_entry.as_ref());

                let extend_configuration_file_path = if extend_entry_as_path.starts_with(".") {
                    relative_resolution_base_path.join(extend_entry.as_ref())
                } else {
                    const RESOLVE_OPTIONS: ResolveOptions = ResolveOptions::new()
                        .with_assume_relative()
                        .with_condition_names(&["biome", "default"]);

                    resolve(
                        extend_entry.as_ref(),
                        external_resolution_base_path,
                        fs,
                        &RESOLVE_OPTIONS,
                    )
                    .map_err(|error| {
                        CantResolve::new(Utf8PathBuf::from(extend_entry), error)
                            .with_verbose_advice(markup! {
                                "Biome tried to resolve the configuration file \""<Emphasis>{
                                    extend_entry
                                }</Emphasis>"\" in \"extends\" using \""<Emphasis>{
                                    external_resolution_base_path.to_string()
                                }</Emphasis>"\" as the base path."
                            })
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
                    CantLoadExtendFile::new(
                        extend_configuration_file_path.to_string(),
                        err.to_string(),
                    )
                    .with_verbose_advice(markup! {
                        "It's possible that the file was created with a "
                        "different user/group. Make sure you have the rights "
                        "to read the file."
                    })
                })?;
                let deserialized = deserialize_from_json_str::<Self>(
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
        }
        Ok(deserialized_configurations)
    }

    /// Checks for the presence of deprecated fields and updates the
    /// configuration to apply them to the new schema.
    fn migrate_deprecated_fields(&mut self) {}
}

#[cfg(test)]
mod test {
    use crate::{WorkspaceError, configuration::load_configuration};
    use biome_configuration::{
        BiomeDiagnostic, ConfigurationPathHint, diagnostics::ConfigurationDiagnostic,
    };
    use biome_fs::MemoryFileSystem;
    use camino::Utf8PathBuf;

    #[test]
    fn should_not_load_a_configuration_yml() {
        let fs = MemoryFileSystem::default();
        fs.insert(Utf8PathBuf::from("biome.yml"), "content".to_string());
        let path_hint = ConfigurationPathHint::FromUser(Utf8PathBuf::from("biome.yml"));

        let result = load_configuration(&fs, path_hint);

        assert!(result.is_err());
    }

    #[test]
    fn should_skip_non_root_configuration() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/biome.json"),
            r#"{ "linter": { "enabled": false } }"#.to_string(),
        );
        fs.insert(
            Utf8PathBuf::from("/nested/biome.json"),
            r#"{ "root": false, "linter": { "enabled": true } }"#.to_string(),
        );
        let path_hint = ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/nested"));

        match load_configuration(&fs, path_hint) {
            Ok(loaded) => {
                assert!(
                    loaded
                        .configuration
                        .linter
                        .is_some_and(|linter| !linter.is_enabled())
                );
            }
            Err(err) => {
                panic!("Config loading failed: {err}");
            }
        }
    }

    #[test]
    fn should_refuse_user_provided_non_root_configuration() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/biome.json"),
            r#"{ "linter": { "enabled": false } }"#.to_string(),
        );
        fs.insert(
            Utf8PathBuf::from("/nested/biome.json"),
            r#"{ "root": false, "linter": { "enabled": true } }"#.to_string(),
        );
        let path_hint = ConfigurationPathHint::FromUser(Utf8PathBuf::from("/nested"));

        match load_configuration(&fs, path_hint) {
            Ok(_) => panic!("Config loading should have failed"),
            Err(err) => {
                assert!(matches!(
                    err,
                    WorkspaceError::Configuration(ConfigurationDiagnostic::Biome(
                        BiomeDiagnostic::NonRootConfiguration(_)
                    ))
                ));
            }
        }
    }
}

/// Use this type to determine what kind of [ScanKind] needs to be used based
/// on the current configuration
pub struct ProjectScanComputer<'a> {
    requires_project_scan: bool,
    enabled_rules: FxHashSet<RuleFilter<'a>>,
    configuration: &'a Configuration,
    skip: &'a [AnalyzerSelector],
    only: &'a [AnalyzerSelector],
}

impl<'a> ProjectScanComputer<'a> {
    pub fn new(configuration: &'a Configuration) -> Self {
        let enabled_rules = configuration.get_linter_rules().as_enabled_rules();
        Self {
            enabled_rules,
            requires_project_scan: false,
            configuration,
            skip: &[],
            only: &[],
        }
    }

    pub fn with_rule_selectors(
        mut self,
        skip: &'a [AnalyzerSelector],
        only: &'a [AnalyzerSelector],
    ) -> Self {
        self.skip = skip;
        self.only = only;
        self
    }

    /// Computes and return the [ScanKind] required by this project
    pub fn compute(mut self) -> ScanKind {
        let domains = self.configuration.get_linter_domains();

        if let Some(domains) = domains {
            for (domain, value) in domains.iter() {
                if domain == &RuleDomain::Project && value != &RuleDomainValue::None {
                    self.requires_project_scan = true;
                    break;
                }
            }
        }

        biome_graphql_analyze::visit_registry(&mut self);
        biome_css_analyze::visit_registry(&mut self);
        biome_json_analyze::visit_registry(&mut self);
        biome_js_analyze::visit_registry(&mut self);

        if self.requires_project_scan {
            ScanKind::Project
        } else {
            // There's no need to scan further known files if the VCS isn't enabled
            if !self.configuration.use_ignore_file() {
                ScanKind::NoScanner
            } else {
                ScanKind::KnownFiles
            }
        }
    }

    fn check_rule<R, L>(&mut self)
    where
        L: Language,
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let filter = RuleFilter::Rule(<R::Group as RuleGroup>::NAME, R::METADATA.name);

        if !self.only.is_empty() {
            for selector in self.only.iter() {
                if selector.match_rule::<R>() {
                    let domains = R::METADATA.domains;
                    self.requires_project_scan |= domains.contains(&RuleDomain::Project);
                    break;
                }
            }
        } else if !self.skip.iter().any(|s| s.match_rule::<R>())
            && self.enabled_rules.contains(&filter)
        {
            let domains = R::METADATA.domains;
            self.requires_project_scan |= domains.contains(&RuleDomain::Project);
        }
    }
}

impl RegistryVisitor<JsLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.check_rule::<R, JsLanguage>();
    }
}

impl RegistryVisitor<JsonLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, JsonLanguage>();
    }
}

impl RegistryVisitor<CssLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, CssLanguage>();
    }
}

impl RegistryVisitor<GraphqlLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, GraphqlLanguage>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_configuration::analyzer::{
        Correctness, DomainSelector, RuleDomainValue, RuleDomains, RuleSelector, SeverityOrGroup,
    };
    use biome_configuration::{
        LinterConfiguration, RuleConfiguration, RulePlainConfiguration, Rules,
    };
    use rustc_hash::FxHashMap;

    #[test]
    fn should_return_none_if_the_linter_is_disabled() {
        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                enabled: Some(false.into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration).compute(),
            ScanKind::NoScanner
        );
    }

    #[test]
    fn should_scan_project_project_domain_is_enabled() {
        let mut domains = FxHashMap::default();
        domains.insert(RuleDomain::Project, RuleDomainValue::Recommended);

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                domains: Some(RuleDomains(domains)),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration).compute(),
            ScanKind::Project
        );
    }

    #[test]
    fn should_scan_project_project_rule_is_enabled() {
        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    correctness: Some(SeverityOrGroup::Group(Correctness {
                        no_private_imports: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Error,
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration).compute(),
            ScanKind::Project
        );
    }

    #[test]
    fn should_skip_project_rule_is_skipped() {
        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    correctness: Some(SeverityOrGroup::Group(Correctness {
                        no_private_imports: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Error,
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration)
                .with_rule_selectors(
                    &[RuleSelector::Rule("correctness", "noPrivateImports").into()],
                    &[]
                )
                .compute(),
            ScanKind::NoScanner
        );
    }

    #[test]
    fn should_return_project_if_project_rule_is_only() {
        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    correctness: Some(SeverityOrGroup::Group(Correctness {
                        no_private_imports: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Off,
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration)
                .with_rule_selectors(
                    &[],
                    &[RuleSelector::Rule("correctness", "noPrivateImports").into()]
                )
                .compute(),
            ScanKind::Project
        );
    }

    #[test]
    fn should_return_project_if_a_domain_contains_project_rules() {
        let configuration = Configuration::default();

        assert_eq!(
            ProjectScanComputer::new(&configuration)
                .with_rule_selectors(&[], &[DomainSelector("project").into()])
                .compute(),
            ScanKind::Project
        );
    }

    #[test]
    fn should_not_return_project_if_a_domain_does_not_contain_project_rules() {
        let configuration = Configuration::default();

        assert_eq!(
            ProjectScanComputer::new(&configuration)
                .with_rule_selectors(&[], &[DomainSelector("test").into()])
                .compute(),
            ScanKind::NoScanner
        );
    }
}
