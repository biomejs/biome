use crate::WorkspaceError;
use crate::settings::Settings;
use crate::workspace::ScanKind;
use biome_analyze::{
    AnalyzerRules, Queryable, RegistryVisitor, Rule, RuleDomain, RuleFilter, RuleGroup,
    RuleMetadata,
};
use biome_configuration::analyzer::{AnalyzerSelector, RuleDomainValue};
use biome_configuration::diagnostics::{
    CantLoadExtendFile, CantResolve, EditorConfigDiagnostic, ParseFailedDiagnostic,
};
use biome_configuration::editorconfig::EditorConfig;
use biome_configuration::{
    BiomeDiagnostic, Configuration, ConfigurationPathHint, ConfigurationSource,
    ConfigurationSourceEntry, ExtendedConfiguration, ExtendedConfigurations, VERSION,
    push_to_analyzer_assist, push_to_analyzer_rules,
};
use biome_console::markup;
#[cfg(feature = "lang_css")]
use biome_css_analyze::METADATA as css_lint_metadata;
#[cfg(feature = "lang_css")]
use biome_css_syntax::CssLanguage;
use biome_deserialize::Deserialized;
use biome_deserialize::json::deserialize_from_json_str;
use biome_diagnostics::{Advices, Diagnostic, DiagnosticExt, Error, LogCategory, Severity, Visit};
use biome_fs::ManifestName;
use biome_fs::{AutoSearchResult, ConfigName, FileSystem, OpenOptions, normalize_path};
#[cfg(feature = "lang_graphql")]
use biome_graphql_analyze::METADATA as graphql_lint_metadata;
#[cfg(feature = "lang_graphql")]
use biome_graphql_syntax::GraphqlLanguage;
#[cfg(feature = "lang_html")]
use biome_html_analyze::METADATA as html_lint_metadata;
#[cfg(feature = "lang_html")]
use biome_html_syntax::HtmlLanguage;
#[cfg(feature = "lang_js")]
use biome_js_analyze::METADATA as js_lint_metadata;
#[cfg(feature = "lang_js")]
use biome_js_syntax::JsLanguage;
use biome_json_analyze::METADATA as json_lint_metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{JsonParserOptions, parse_json};
use biome_json_syntax::JsonLanguage;
use biome_manifest::{BiomeManifest, ManifestEntry};
#[cfg(feature = "lang_md")]
use biome_markdown_analyze::METADATA as md_lint_metadata;
#[cfg(feature = "lang_md")]
use biome_markdown_syntax::MarkdownLanguage;
use biome_resolver::{
    FsWithResolverProxy, PathInfo, ResolveOptions, is_relative_specifier,
    package_has_exported_subpath, package_specifier_parts, resolve, resolve_package_root,
};
use biome_rowan::Language;
use camino::{Utf8Path, Utf8PathBuf};
use rustc_hash::{FxHashMap, FxHashSet};
use std::borrow::Cow;
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::ErrorKind;
use std::iter::FusedIterator;
use std::ops::Deref;
use std::str::FromStr;
use std::sync::Arc;
use tracing::instrument;

fn display_path(path: &Utf8Path) -> Cow<'_, str> {
    if cfg!(any(test, windows)) {
        Cow::Owned(path.as_str().replace('\\', "/"))
    } else {
        Cow::Borrowed(path.as_str())
    }
}

/// Information regarding the configuration inputs that were found.
#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    /// The unmerged root and extended configuration inputs.
    pub source: ConfigurationSource,
    /// All diagnostics emitted while loading and resolving the configuration.
    pub diagnostics: Vec<Error>,
    /// Where the configuration was loaded from.
    pub loaded_location: LoadedLocation,
}

#[derive(Default, Debug)]
pub enum LoadedLocation {
    /// Loaded from inside the project
    #[default]
    InProject,
    /// Loaded from a parent folder
    ParentFolder,
    /// Loaded from the user configuration folder
    UserConfigFolder,
}

impl LoadedLocation {
    pub const fn is_in_project(&self) -> bool {
        matches!(self, Self::InProject)
    }
}

impl LoadedConfiguration {
    /// Consumes a payload and loads its extended configuration inputs.
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
            loaded_location,
            source,
        } = value;
        let source: Arc<str> = source.into();
        let (partial_configuration, diagnostics) = deserialized.consume();
        let mut diagnostics = diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.with_file_path(configuration_file_path.to_string()))
            .collect::<Vec<_>>();
        let mut partial_configuration = partial_configuration;
        let extended_configurations = match partial_configuration.as_mut() {
            Some(partial_configuration) => {
                let extended_configurations = ConfigurationExtendsLoader::new(fs, &mut diagnostics)
                    .load(
                        partial_configuration,
                        &configuration_file_path,
                        &external_resolution_base_path,
                    )?;
                // Normalize plugin paths relative to the configuration file directory so
                // merged configurations (e.g. nested configs extending from root) can
                // still load plugins defined in other configuration files.
                #[cfg(feature = "plugins")]
                {
                    let config_dir = configuration_file_path
                        .parent()
                        .unwrap_or(external_resolution_base_path.as_path());
                    if let Some(plugins) = partial_configuration.plugins.as_mut() {
                        plugins
                            .normalize_relative_paths(fs, config_dir)
                            .map_err(|diagnostic| {
                                WorkspaceError::plugin_errors(vec![diagnostic])
                            })?;
                    }
                    if let Some(overrides) = partial_configuration.overrides.as_mut() {
                        for pattern in overrides.0.iter_mut() {
                            if let Some(plugins) = pattern.plugins.as_mut() {
                                plugins.normalize_relative_paths(fs, config_dir).map_err(
                                    |diagnostic| WorkspaceError::plugin_errors(vec![diagnostic]),
                                )?;
                            }
                        }
                    }
                }
                extended_configurations
            }
            None => Vec::new(),
        };

        let directory_path = configuration_file_path.parent().map(Utf8PathBuf::from);

        Ok(Self {
            source: ConfigurationSource {
                directory_path,
                root: Some(ConfigurationSourceEntry {
                    configuration: partial_configuration,
                    file_path: Some(configuration_file_path),
                    file_source: Some(source),
                }),
                extended_configurations: ExtendedConfigurations::from(extended_configurations),
            },
            diagnostics,
            loaded_location,
        })
    }

    /// Resolves the loaded configuration inputs.
    pub fn resolved_configuration(&self) -> Configuration {
        self.source.resolve()
    }

    /// Returns the extended configurations with their resolved file paths.
    pub fn extended_configurations(&self) -> Vec<(Utf8PathBuf, Configuration)> {
        self.source
            .extended_configurations
            .as_slice()
            .iter()
            .filter_map(|extended| {
                Some((
                    extended.source.file_path.clone()?,
                    extended.source.configuration.clone()?,
                ))
            })
            .collect()
    }

    /// Return the path of the **directory** where the configuration is
    pub fn directory_path(&self) -> Option<&Utf8Path> {
        self.source.directory_path.as_deref()
    }

    /// Return the path of the **file** where the configuration is
    pub fn file_path(&self) -> Option<&Utf8Path> {
        self.source.root.as_ref()?.file_path.as_deref()
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

#[derive(Debug)]
pub struct ConfigurationPayload {
    /// The result of the deserialization
    pub deserialized: Deserialized<Configuration>,
    /// The path of where the `biome.json` or `biome.jsonc` file was found. This contains the file name.
    pub configuration_file_path: Utf8PathBuf,
    /// The base path where the external configuration in a package should be resolved from
    pub external_resolution_base_path: Utf8PathBuf,
    /// The exact source text used to deserialize the configuration.
    pub source: String,

    pub loaded_location: LoadedLocation,
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
        ConfigurationPathHint::FromUserExternal(path) => path.clone(),
        ConfigurationPathHint::None => fs.working_directory().unwrap_or_default(),
    };

    // If the configuration path hint is not a file path
    // we'll auto search for the configuration file
    let configuration_directory = match path_hint {
        ConfigurationPathHint::FromLsp(path) => path,
        ConfigurationPathHint::FromWorkspace(path) => path,
        ConfigurationPathHint::FromUser(ref config_file_path)
        | ConfigurationPathHint::FromUserExternal(ref config_file_path) => {
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

    let Some((auto_search_result, loaded_location)) = fs
        .auto_search_files_with_predicate(
            &configuration_directory,
            &ConfigName::file_names(),
            &mut predicate,
        )
        .map(|auto_search_result| {
            if configuration_directory == auto_search_result.directory_path {
                (auto_search_result, LoadedLocation::InProject)
            } else {
                (auto_search_result, LoadedLocation::ParentFolder)
            }
        })
        .or_else(|| {
            let user_config_dir = fs.user_config_dir()?;

            let paths = ConfigName::file_names().map(|file_name| user_config_dir.join(file_name));
            fs.read_file_from_paths_with_predicate(paths.as_slice(), &mut predicate)
                .map(|auto_search_result| (auto_search_result, LoadedLocation::UserConfigFolder))
        })
    else {
        return Ok(None);
    };

    Ok(Some(ConfigurationPayload {
        // SAFETY: unwrapping is safe because the predicate in the search above would
        // only return `true` if it assigned `Some` value:
        deserialized: deserialized.unwrap(),
        configuration_file_path: auto_search_result.file_path,
        external_resolution_base_path,
        source: auto_search_result.content,
        loaded_location,
    }))
}

fn load_user_config(
    fs: &dyn FileSystem,
    config_file_path: &Utf8Path,
    external_resolution_base_path: Utf8PathBuf,
) -> LoadConfig {
    // If the configuration path hint is a file path, we'll load it directly.
    let working_directory = fs.working_directory();
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
        let loaded_location = working_directory.map_or(LoadedLocation::InProject, |wd| {
            if config_file_path.starts_with(wd) {
                LoadedLocation::InProject
            } else {
                LoadedLocation::ParentFolder
            }
        });
        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: config_file_path.to_path_buf(),
            external_resolution_base_path,
            source: content,
            loaded_location,
        }))
    } else {
        let config_paths =
            ConfigName::file_names().map(|file_name| config_file_path.join(file_name));
        let result = fs.read_files_from_paths(config_paths.as_slice());
        let Some(result) = result else {
            return Err(BiomeDiagnostic::no_configuration_file_found(config_file_path).into());
        };

        let parser_options = if result.file_path.extension() == Some("json") {
            JsonParserOptions::default()
        } else {
            JsonParserOptions::default()
                .with_allow_comments()
                .with_allow_trailing_commas()
        };

        let content = fs.read_file_from_path(result.file_path.as_path())?;
        let deserialized = deserialize_from_json_str::<Configuration>(&content, parser_options, "");
        if deserialized
            .deserialized
            .as_ref()
            .is_some_and(|config| config.root.is_some_and(|root| !root.value()))
        {
            return Err(BiomeDiagnostic::non_root_configuration(config_file_path).into());
        }
        let loaded_location = working_directory.map_or(LoadedLocation::InProject, |wd| {
            if config_file_path.starts_with(wd) {
                LoadedLocation::InProject
            } else {
                LoadedLocation::ParentFolder
            }
        });
        Ok(Some(ConfigurationPayload {
            deserialized,
            configuration_file_path: result.file_path.to_path_buf(),
            external_resolution_base_path,
            source: content,
            loaded_location,
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
    let override_indices = settings.matching_override_indices(path);
    to_analyzer_rules_by_indices(settings, &override_indices)
}

pub(crate) fn to_analyzer_rules_by_indices(
    settings: &Settings,
    override_indices: &[usize],
) -> AnalyzerRules {
    let mut analyzer_rules = AnalyzerRules::default();
    if let Some(rules) = settings.linter.rules.as_ref() {
        #[cfg(feature = "lang_js")]
        push_to_analyzer_rules(rules, js_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_css")]
        push_to_analyzer_rules(rules, css_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_rules(rules, json_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_graphql")]
        push_to_analyzer_rules(rules, graphql_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_html")]
        push_to_analyzer_rules(rules, html_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_md")]
        push_to_analyzer_rules(rules, md_lint_metadata.deref(), &mut analyzer_rules);
    }
    if let Some(rules) = settings.assist.actions.as_ref() {
        #[cfg(feature = "lang_js")]
        push_to_analyzer_assist(rules, js_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_css")]
        push_to_analyzer_assist(rules, css_lint_metadata.deref(), &mut analyzer_rules);
        push_to_analyzer_assist(rules, json_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_graphql")]
        push_to_analyzer_assist(rules, graphql_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_html")]
        push_to_analyzer_assist(rules, html_lint_metadata.deref(), &mut analyzer_rules);
        #[cfg(feature = "lang_md")]
        push_to_analyzer_assist(rules, md_lint_metadata.deref(), &mut analyzer_rules);
    }
    settings
        .override_settings
        .override_analyzer_rules_by_indices(override_indices, analyzer_rules)
}

const MAX_EXTENDS_DEPTH: usize = 10;
const MAX_MANIFEST_IMPORT_DEPTH: usize = 10;

/// Loads a configuration's complete `extends` graph in dependency-first merge order.
struct ConfigurationExtendsLoader<'a> {
    fs: &'a dyn FsWithResolverProxy,
    diagnostics: &'a mut Vec<Error>,
    package_resolutions: FxHashMap<String, PackageResolution>,
}

impl<'a> ConfigurationExtendsLoader<'a> {
    fn new(fs: &'a dyn FsWithResolverProxy, diagnostics: &'a mut Vec<Error>) -> Self {
        Self {
            fs,
            diagnostics,
            package_resolutions: FxHashMap::default(),
        }
    }

    /// Returns extended inputs in dependency-first merge order.
    ///
    /// Multiple versions of an extended package or an eleventh level are reported as errors and
    /// aren't loaded.
    fn load(
        mut self,
        root: &Configuration,
        root_file_path: &Utf8Path,
        root_external_resolution_base_path: &Utf8Path,
    ) -> Result<Vec<ExtendedConfiguration>, WorkspaceError> {
        let mut pending_configurations = vec![PendingConfiguration::root(
            root,
            root_file_path.to_path_buf(),
            root_external_resolution_base_path.to_path_buf(),
        )];
        let mut extended_configurations = Vec::new();

        while let Some(pending) = pending_configurations.last_mut() {
            let Some(specifier) = pending.next_specifier() else {
                let completed = pending_configurations
                    .pop()
                    .expect("the pending configuration should exist");
                if let Some(configuration) = completed.configuration {
                    extended_configurations.push(configuration);
                }
                continue;
            };

            let resolved = self.resolve_extended_configuration(pending, specifier)?;

            if let Some(first) = self.conflicting_package_resolution(&resolved) {
                self.report_conflict(first, resolved);
                continue;
            }

            let depth = pending.depth + 1;
            if depth > MAX_EXTENDS_DEPTH {
                self.diagnostics.push(
                    ExtendsDepthLimit {
                        path: display_path(&pending.file_path).into_owned(),
                        specifier: resolved.specifier.clone().into(),
                    }
                    .into(),
                );
                continue;
            }

            self.register_package_resolution(&resolved);
            let mut loaded = self.load_resolved_configuration(resolved)?;
            self.diagnostics.append(&mut loaded.diagnostics);
            pending_configurations.push(PendingConfiguration::extended(
                loaded,
                depth,
                root_external_resolution_base_path,
            ));
        }

        Ok(extended_configurations)
    }

    fn report_conflict(
        &mut self,
        first: PackageResolution,
        resolved: ResolvedExtendedConfiguration,
    ) {
        self.diagnostics.push(
            MultipleExtendedConfigurationVersions {
                specifier: resolved.identity().to_string(),
                path: display_path(&resolved.file_path).into_owned(),
                advice: MultipleExtendedConfigurationVersionsAdvice {
                    first,
                    conflicting: PackageResolution::from(&resolved),
                },
            }
            .into(),
        );
    }

    fn conflicting_package_resolution(
        &self,
        resolved: &ResolvedExtendedConfiguration,
    ) -> Option<PackageResolution> {
        let identity = resolved.package_identity()?;
        let first = self.package_resolutions.get(identity)?;
        (first.version() != resolved.package_version()).then(|| first.clone())
    }

    fn register_package_resolution(&mut self, resolved: &ResolvedExtendedConfiguration) {
        if let Some(identity) = resolved.package_identity() {
            self.package_resolutions
                .entry(identity.to_string())
                .or_insert_with(|| PackageResolution::from(resolved));
        }
    }

    /// Resolves one `extends` entry without reading or deserializing its configuration file.
    ///
    /// Relative entries use the declaring configuration as their base. Package entries use the
    /// external resolution base and honor explicit `configs/*` exports before falling back to a
    /// named export from a Biome manifest. Manifest exports retain the selected package identity
    /// even when the exported file comes from another package.
    ///
    /// Returns an error for invalid manifest selectors, blocked or broken package exports, and
    /// unresolved paths.
    fn resolve_extended_configuration(
        &self,
        pending: &PendingConfiguration,
        specifier: String,
    ) -> Result<ResolvedExtendedConfiguration, WorkspaceError> {
        const RESOLVE_OPTIONS: ResolveOptions = ResolveOptions::new()
            .with_assume_relative()
            .with_condition_names(&["biome", "default"]);
        let cant_resolve = |error| {
            CantResolve::new(Utf8PathBuf::from(&specifier), error).with_verbose_advice(markup! {
                "Biome tried to resolve the configuration file \""<Emphasis>{
                    &specifier
                }</Emphasis>"\" in \"extends\" using \""<Emphasis>{
                    pending.external_resolution_base_path.to_string()
                }</Emphasis>"\" as the base path."
            })
        };

        let named_config = match package_specifier_parts(&specifier) {
            Ok((package, subpath)) => match subpath.strip_prefix("configs/") {
                Some(name) if !name.is_empty() && !name.contains('/') => Some((package, name)),
                Some(_) => {
                    return Err(CantResolve::new(
                        Utf8PathBuf::from(&specifier),
                        biome_resolver::ResolveError::InvalidPackageSpecifier,
                    )
                    .with_message(markup! {
                        "Manifest configuration selections must identify one configuration after the "
                        <Emphasis>"configs/"</Emphasis>" subpath."
                    })
                    .into());
                }
                None => None,
            },
            Err(_) => None,
        };
        let (file_path, is_named_manifest_config, selected_package) =
            if let Some((package, name)) = named_config {
                let resolved = resolve(
                    &specifier,
                    &pending.external_resolution_base_path,
                    self.fs,
                    &RESOLVE_OPTIONS,
                );
                match resolved {
                    Ok(path) if !ManifestName::is_manifest_file(&path) => (path, false, None),
                    result => {
                        let subpath = format!("configs/{name}");
                        if package_has_exported_subpath(
                            package,
                            &subpath,
                            &pending.external_resolution_base_path,
                            self.fs,
                        )
                        .map_err(&cant_resolve)?
                        {
                            (result.map_err(&cant_resolve)?, false, None)
                        } else {
                            let (path, package) = self.resolve_named_manifest_config(
                                package,
                                name,
                                &specifier,
                                &pending.external_resolution_base_path,
                            )?;
                            (path, true, package)
                        }
                    }
                }
            } else if is_relative_specifier(&specifier) {
                (
                    normalize_path(&pending.relative_resolution_base_path.join(&specifier)),
                    false,
                    None,
                )
            } else {
                (
                    resolve(
                        &specifier,
                        &pending.external_resolution_base_path,
                        self.fs,
                        &RESOLVE_OPTIONS,
                    )
                    .map_err(cant_resolve)?,
                    false,
                    None,
                )
            };

        let file_path = match self.fs.path_info(&file_path) {
            Ok(PathInfo::Symlink {
                canonicalized_target,
            }) => canonicalized_target,
            _ => normalize_path(&file_path),
        };
        if !is_named_manifest_config && ManifestName::is_manifest_file(&file_path) {
            return Err(CantResolve::new(
                Utf8PathBuf::from(&specifier),
                biome_resolver::ResolveError::InvalidPackageSpecifier,
            )
            .with_message(markup! {
                "Biome manifests must select a named configuration with the "
                <Emphasis>"configs/"</Emphasis>" subpath."
            })
            .into());
        }
        let package = selected_package.or_else(|| {
            file_path
                .parent()
                .and_then(|parent| self.fs.find_package_json(parent).ok())
                .map(|(_, manifest)| ResolvedPackage {
                    name: manifest.name.map(String::from),
                    version: manifest.version.map(String::from),
                })
                .filter(|package| package.matches_specifier(&specifier))
        });

        Ok(ResolvedExtendedConfiguration {
            reference: ConfigurationReference {
                from: pending.file_path.clone(),
            },
            specifier,
            file_path,
            package,
        })
    }

    /// Resolves a named configuration exported by a package's Biome manifest.
    ///
    /// The returned path may point into another package when the selected package re-exports a
    /// configuration. The accompanying package metadata always identifies the package selected by
    /// the caller so version-conflict detection does not depend on the leaf file location.
    fn resolve_named_manifest_config(
        &self,
        package: &str,
        name: &str,
        specifier: &str,
        base_path: &Utf8Path,
    ) -> Result<(Utf8PathBuf, Option<ResolvedPackage>), WorkspaceError> {
        let manifest_path = self.resolve_package_biome_manifest(package, base_path)?;
        let resolved_package = manifest_path
            .parent()
            .and_then(|parent| self.fs.find_package_json(parent).ok())
            .map(|(_, manifest)| ResolvedPackage {
                name: manifest.name.map(String::from),
                version: manifest.version.map(String::from),
            })
            .filter(|resolved| resolved.matches_specifier(package));
        let manifest = BiomeManifest::load(self.fs, &manifest_path)?;
        let configs = self.collect_manifest_configs(&manifest_path, manifest.configs)?;
        Self::exported_config_key(&configs, name)
            .and_then(|key| configs.get(key))
            .map(|config| (config.path.clone(), resolved_package))
            .ok_or_else(|| {
                CantResolve::new(
                    Utf8PathBuf::from(specifier),
                    biome_resolver::ResolveError::NotFound,
                )
                .with_message(markup! {
                    "The Biome manifest for "<Emphasis>{package}</Emphasis>
                    " does not export a configuration named "<Emphasis>{name}</Emphasis>"."
                })
                .into()
            })
    }

    /// Collects the configurations exported by a manifest and its selected package imports.
    ///
    /// Imports are traversed iteratively up to [`MAX_MANIFEST_IMPORT_DEPTH`]. Imported exports keep
    /// their immediate source package in the internal key and expose their unqualified name at the
    /// current package boundary. Active cycle edges are skipped while the active manifest continues
    /// collecting its remaining entries.
    ///
    /// Returns an error for unresolved selections, duplicate public names, invalid local paths, or
    /// imports beyond the depth limit.
    fn collect_manifest_configs(
        &self,
        manifest_path: &Utf8Path,
        entries: Vec<ManifestEntry>,
    ) -> Result<BTreeMap<String, ManifestConfig>, WorkspaceError> {
        struct PendingManifest {
            path: Utf8PathBuf,
            entries: std::vec::IntoIter<ManifestEntry>,
            configs: BTreeMap<String, ManifestConfig>,
            import: Option<PendingImport>,
        }

        struct PendingImport {
            package: String,
            selection: String,
        }

        let mut active_manifests = FxHashSet::default();
        active_manifests.insert(manifest_path.to_path_buf());
        let mut pending = vec![PendingManifest {
            path: manifest_path.to_path_buf(),
            entries: entries.into_iter(),
            configs: BTreeMap::new(),
            import: None,
        }];

        while let Some(current) = pending.last_mut() {
            let Some(entry) = current.entries.next() else {
                let Some(mut completed) = pending.pop() else {
                    break;
                };
                active_manifests.remove(&completed.path);
                let Some(import) = completed.import else {
                    return Ok(completed.configs);
                };
                let config =
                    Self::remove_exported_config(&mut completed.configs, &import.selection)
                        .ok_or_else(|| {
                            CantResolve::new(
                        Utf8PathBuf::from(&import.package),
                        biome_resolver::ResolveError::NotFound,
                    )
                    .with_message(markup! {
                        "Biome manifest for "<Emphasis>{import.package}</Emphasis>
                        " does not export configuration "<Emphasis>{import.selection}</Emphasis>"."
                    })
                        })?;
                let Some(parent) = pending.last_mut() else {
                    break;
                };
                let export_name = format!("{}/{}", import.package, config.export_name);
                Self::insert_named_config(&mut parent.configs, &parent.path, export_name, config)?;
                continue;
            };
            let current_path = current.path.clone();
            let manifest_dir = current_path.parent().unwrap_or(Utf8Path::new(""));

            match entry {
                ManifestEntry::Package(specifier) => {
                    let (package, selection) = package_specifier_parts(&specifier)
                        .map_err(|error| CantResolve::new(Utf8PathBuf::from(&specifier), error))?;
                    let selection = selection.strip_prefix("configs/").ok_or_else(|| {
                        CantResolve::new(
                            Utf8PathBuf::from(&specifier),
                            biome_resolver::ResolveError::InvalidPackageSpecifier,
                        )
                        .with_message(markup! {
                            "Manifest configuration imports must select a named configuration with the "
                            <Emphasis>"configs/"</Emphasis>" subpath."
                        })
                    })?;
                    if selection.is_empty() || selection.contains('/') {
                        return Err(CantResolve::new(
                            Utf8PathBuf::from(&specifier),
                            biome_resolver::ResolveError::InvalidPackageSpecifier,
                        )
                        .with_message(markup! {
                            "Manifest configuration imports must identify one configuration after the "
                            <Emphasis>"configs/"</Emphasis>" subpath."
                        })
                        .into());
                    }
                    if pending.len() > MAX_MANIFEST_IMPORT_DEPTH {
                        return Err(CantResolve::new(
                            current_path,
                            biome_resolver::ResolveError::InvalidMappingTarget,
                        )
                        .with_message(markup! {
                            "Biome manifest imports must not exceed "{MAX_MANIFEST_IMPORT_DEPTH}" levels."
                        })
                        .into());
                    }
                    let imported_path =
                        self.resolve_package_biome_manifest(package, manifest_dir)?;
                    if !active_manifests.insert(imported_path.clone()) {
                        // Skip only the cyclic edge. The active manifest continues collecting its
                        // remaining entries in dependency-first order.
                        continue;
                    }
                    let imported = BiomeManifest::load(self.fs, &imported_path)?;
                    pending.push(PendingManifest {
                        path: imported_path,
                        entries: imported.configs.into_iter(),
                        configs: BTreeMap::new(),
                        import: Some(PendingImport {
                            package: package.to_string(),
                            selection: selection.to_string(),
                        }),
                    });
                }
                ManifestEntry::Paths(paths) => {
                    let Some(current) = pending.last_mut() else {
                        break;
                    };
                    for (name, path) in paths {
                        if name.is_empty() || name.contains('/') {
                            return Err(CantResolve::new(
                                current_path.clone(),
                                biome_resolver::ResolveError::InvalidPackageSpecifier,
                            )
                            .with_message(markup! {
                                "Biome manifest configuration names must not be empty or contain slashes. Invalid name: "
                                <Emphasis>{name}</Emphasis>"."
                            })
                            .into());
                        }
                        let path =
                            Self::resolve_manifest_config_path(self.fs, manifest_dir, &path)?;
                        Self::insert_named_config(
                            &mut current.configs,
                            &current_path,
                            name.clone(),
                            ManifestConfig {
                                path,
                                export_name: name,
                            },
                        )?;
                    }
                }
            }
        }

        Ok(BTreeMap::new())
    }

    /// Inserts a configuration after verifying both its internal key and public export name are
    /// unique within the manifest.
    ///
    /// Imported configurations may have different source-qualified keys but still collide after
    /// rebinding to the current package's public namespace.
    fn insert_named_config(
        configs: &mut BTreeMap<String, ManifestConfig>,
        manifest_path: &Utf8Path,
        name: String,
        config: ManifestConfig,
    ) -> Result<(), WorkspaceError> {
        if configs.contains_key(&name)
            || configs
                .values()
                .any(|existing| existing.export_name == config.export_name)
        {
            return Err(CantResolve::new(
                manifest_path.to_path_buf(),
                biome_resolver::ResolveError::InvalidPackageSpecifier,
            )
            .with_message(markup! {
                "Biome manifest exports multiple configurations named "
                <Emphasis>{config.export_name}</Emphasis>"."
            })
            .into());
        }
        configs.insert(name, config);
        Ok(())
    }

    fn exported_config_key<'b>(
        configs: &'b BTreeMap<String, ManifestConfig>,
        export_name: &str,
    ) -> Option<&'b str> {
        configs
            .iter()
            .find_map(|(key, config)| (config.export_name == export_name).then_some(key.as_str()))
    }

    fn remove_exported_config(
        configs: &mut BTreeMap<String, ManifestConfig>,
        export_name: &str,
    ) -> Option<ManifestConfig> {
        let key = Self::exported_config_key(configs, export_name)?.to_string();
        configs.remove(&key)
    }

    fn resolve_manifest_config_path(
        fs: &dyn FileSystem,
        manifest_dir: &Utf8Path,
        path: &str,
    ) -> Result<Utf8PathBuf, WorkspaceError> {
        let invalid_path = || {
            CantResolve::new(
                Utf8PathBuf::from(path),
                biome_resolver::ResolveError::InvalidPackageSpecifier,
            )
            .with_message(markup! {
                "Biome manifest configuration paths must stay within the package directory and must not contain symbolic links."
            })
        };
        let relative_path = Utf8Path::new(path);
        let resolved = normalize_path(&manifest_dir.join(relative_path));
        if relative_path.is_absolute() || !resolved.starts_with(manifest_dir) {
            return Err(invalid_path().into());
        }

        let resolved = if resolved.extension().is_some() || fs.path_is_file(&resolved) {
            resolved
        } else {
            ["json", "jsonc"]
                .into_iter()
                .map(|extension| resolved.with_extension(extension))
                .find(|candidate| fs.path_is_file(candidate))
                .unwrap_or(resolved)
        };
        let relative_path = resolved
            .strip_prefix(manifest_dir)
            .map_err(|_| invalid_path())?;
        let mut candidate = manifest_dir.to_path_buf();
        for component in relative_path.components() {
            candidate.push(component.as_str());
            if fs.path_is_symlink(&candidate) {
                return Err(invalid_path().into());
            }
        }

        Ok(resolved)
    }

    fn resolve_package_biome_manifest(
        &self,
        package: &str,
        base_path: &Utf8Path,
    ) -> Result<Utf8PathBuf, WorkspaceError> {
        const RESOLVE_OPTIONS: ResolveOptions =
            ResolveOptions::new().with_condition_names(&["biome", "default"]);

        let resolved = resolve(package, base_path, self.fs, &RESOLVE_OPTIONS);
        let package_root = resolve_package_root(package, base_path, self.fs).map_err(|error| {
            CantResolve::new(Utf8PathBuf::from(package), error).with_verbose_advice(markup! {
                "Biome tried to resolve the Biome manifest for "<Emphasis>{package}</Emphasis>"."
            })
        })?;
        if let Ok(path) = &resolved
            && ManifestName::is_manifest_file(path)
        {
            Self::validate_package_manifest_path(self.fs, &package_root, path, package)?;
            return Ok(path.clone());
        }

        let manifest_path = ManifestName::file_names()
            .iter()
            .map(|name| package_root.join(name))
            .find(|path| self.fs.path_is_file(path))
            .ok_or_else(|| {
                CantResolve::new(
                    Utf8PathBuf::from(package),
                    biome_resolver::ResolveError::NotFound,
                )
                .with_message(markup! {
                    "Package "<Emphasis>{package}</Emphasis>
                    " must provide "<Emphasis>{ManifestName::biome_manifest_json()}</Emphasis>
                    " or "<Emphasis>{ManifestName::biome_manifest_jsonc()}</Emphasis>"."
                })
            })?;
        Self::validate_package_manifest_path(self.fs, &package_root, &manifest_path, package)?;
        Ok(manifest_path)
    }

    fn validate_package_manifest_path(
        fs: &dyn FileSystem,
        package_root: &Utf8Path,
        manifest_path: &Utf8Path,
        package: &str,
    ) -> Result<(), WorkspaceError> {
        let invalid_manifest = || {
            CantResolve::new(
                Utf8PathBuf::from(package),
                biome_resolver::ResolveError::InvalidPackageSpecifier,
            )
            .with_message(markup! {
                "The Biome manifest for "<Emphasis>{package}</Emphasis>
                " must stay within the package directory and must not contain symbolic links."
            })
        };
        let relative_path = manifest_path
            .strip_prefix(package_root)
            .map_err(|_| invalid_manifest())?;
        let mut candidate = package_root.to_path_buf();
        for component in relative_path.components() {
            candidate.push(component.as_str());
            if fs.path_is_symlink(&candidate) {
                return Err(invalid_manifest().into());
            }
        }
        Ok(())
    }

    /// Reads and deserializes a resolved extended configuration.
    ///
    /// File access failures are returned as workspace errors. Deserialization failures are retained
    /// in the returned diagnostics and produce a loaded input without a typed configuration.
    fn load_resolved_configuration(
        &self,
        resolved: ResolvedExtendedConfiguration,
    ) -> Result<LoadedExtendedConfiguration, WorkspaceError> {
        let source = self.read_configuration_source(&resolved.file_path)?;
        let deserialized = deserialize_from_json_str::<Configuration>(
            source.as_str(),
            match resolved.file_path.extension() {
                Some("json") => JsonParserOptions::default(),
                _ => JsonParserOptions::default()
                    .with_allow_comments()
                    .with_allow_trailing_commas(),
            },
            "",
        );
        let (mut configuration, diagnostics) = deserialized.consume();
        if let Some(configuration) = configuration.as_mut() {
            Self::normalize_plugins(
                self.fs,
                configuration,
                &resolved.file_path,
                resolved.file_path.parent().unwrap_or(Utf8Path::new("")),
            )?;
        }
        let diagnostics = diagnostics
            .into_iter()
            .map(|diagnostic| diagnostic.with_file_path(resolved.file_path.to_string()))
            .collect();

        Ok(LoadedExtendedConfiguration {
            specifier: resolved.specifier,
            file_path: resolved.file_path,
            source: source.into(),
            configuration,
            diagnostics,
        })
    }

    fn read_configuration_source(&self, file_path: &Utf8Path) -> Result<String, WorkspaceError> {
        let mut file =
            self.fs
                .open_with_options(file_path, OpenOptions::default().read(true))
                .map_err(|err| {
                    CantLoadExtendFile::new(file_path.to_string(), err.to_string())
                        .with_verbose_advice(markup! {
                            "Biome tried to load the configuration file \""<Emphasis>{
                                file_path.to_string()
                            }</Emphasis>"\" from \"extends\"."
                        })
                })?;

        let mut source = String::new();
        file.read_to_string(&mut source).map_err(|err| {
            CantLoadExtendFile::new(file_path.to_string(), err.to_string()).with_verbose_advice(
                markup! {
                    "It's possible that the file was created with a "
                    "different user/group. Make sure you have the rights "
                    "to read the file."
                },
            )
        })?;
        Ok(source)
    }

    #[cfg(feature = "plugins")]
    fn normalize_plugins(
        fs: &dyn FsWithResolverProxy,
        configuration: &mut Configuration,
        file_path: &Utf8Path,
        fallback_resolution_base_path: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        let config_dir = file_path.parent().unwrap_or(fallback_resolution_base_path);
        if let Some(plugins) = configuration.plugins.as_mut() {
            plugins
                .normalize_object_relative_paths(fs, config_dir)
                .map_err(|diagnostic| WorkspaceError::plugin_errors(vec![diagnostic]))?;
        }
        if let Some(overrides) = configuration.overrides.as_mut() {
            for pattern in overrides.0.iter_mut() {
                if let Some(plugins) = pattern.plugins.as_mut() {
                    plugins
                        .normalize_object_relative_paths(fs, config_dir)
                        .map_err(|diagnostic| WorkspaceError::plugin_errors(vec![diagnostic]))?;
                }
            }
        }
        Ok(())
    }

    #[cfg(not(feature = "plugins"))]
    fn normalize_plugins(
        _fs: &dyn FsWithResolverProxy,
        _configuration: &mut Configuration,
        _file_path: &Utf8Path,
        _fallback_resolution_base_path: &Utf8Path,
    ) -> Result<(), WorkspaceError> {
        Ok(())
    }
}

struct ManifestConfig {
    path: Utf8PathBuf,
    export_name: String,
}

/// A configuration waiting for its nested extended configurations to be processed.
///
/// The loader retains each configuration until all of its `extends` entries have been processed,
/// then emits it after its dependencies. The root configuration participates in traversal but is
/// not emitted as an extended configuration.
struct PendingConfiguration {
    /// The extended configuration emitted after its dependencies, or `None` for the root.
    configuration: Option<ExtendedConfiguration>,
    /// The resolved path used as the origin of nested references and diagnostics.
    file_path: Utf8PathBuf,
    /// The directory used to resolve relative `extends` specifiers.
    relative_resolution_base_path: Utf8PathBuf,
    /// The directory used to resolve package and other non-relative `extends` specifiers.
    external_resolution_base_path: Utf8PathBuf,
    /// The declared `extends` specifiers in source order.
    specifiers: Vec<String>,
    /// The index of the next specifier to process.
    next_specifier: usize,
    /// The number of extended configuration edges from the root.
    depth: usize,
}

impl PendingConfiguration {
    fn root(
        configuration: &Configuration,
        file_path: Utf8PathBuf,
        external_resolution_base_path: Utf8PathBuf,
    ) -> Self {
        let relative_resolution_base_path = file_path
            .parent()
            .unwrap_or(&external_resolution_base_path)
            .to_path_buf();
        Self {
            configuration: None,
            file_path,
            relative_resolution_base_path,
            external_resolution_base_path,
            specifiers: Self::specifiers(configuration),
            next_specifier: 0,
            depth: 0,
        }
    }

    fn extended(
        loaded: LoadedExtendedConfiguration,
        depth: usize,
        fallback_resolution_base_path: &Utf8Path,
    ) -> Self {
        let specifiers = loaded
            .configuration
            .as_ref()
            .map_or_else(Vec::new, Self::specifiers);
        let resolution_base_path = loaded
            .file_path
            .parent()
            .unwrap_or(fallback_resolution_base_path)
            .to_path_buf();
        let file_path = loaded.file_path;
        let configuration = Some(ExtendedConfiguration {
            specifier: Some(loaded.specifier),
            source: ConfigurationSourceEntry {
                configuration: loaded.configuration,
                file_path: Some(file_path.clone()),
                file_source: Some(loaded.source),
            },
        });

        Self {
            configuration,
            file_path,
            relative_resolution_base_path: resolution_base_path.clone(),
            external_resolution_base_path: resolution_base_path,
            specifiers,
            next_specifier: 0,
            depth,
        }
    }

    fn specifiers(configuration: &Configuration) -> Vec<String> {
        configuration
            .extends
            .as_ref()
            .and_then(|extends| extends.as_list())
            .map(|extends| extends.iter().map(|entry| entry.to_string()).collect())
            .unwrap_or_default()
    }

    fn next_specifier(&mut self) -> Option<String> {
        let specifier = self.specifiers.get(self.next_specifier)?.clone();
        self.next_specifier += 1;
        Some(specifier)
    }
}

struct ResolvedExtendedConfiguration {
    specifier: String,
    file_path: Utf8PathBuf,
    package: Option<ResolvedPackage>,
    reference: ConfigurationReference,
}

impl ResolvedExtendedConfiguration {
    fn identity(&self) -> &str {
        self.package_identity()
            .unwrap_or_else(|| self.file_path.as_str())
    }

    fn package_identity(&self) -> Option<&str> {
        self.package
            .as_ref()
            .and_then(|package| package.name.as_deref())
    }

    fn package_version(&self) -> Option<&str> {
        self.package
            .as_ref()
            .and_then(|package| package.version.as_deref())
    }
}

/// A resolved and readable extended configuration file.
///
/// The path and source identify the loaded file independently of typed deserialization.
/// [`Self::configuration`] is `None` when deserialization could not produce a configuration, and
/// [`Self::diagnostics`] describes the failure.
struct LoadedExtendedConfiguration {
    specifier: String,
    file_path: Utf8PathBuf,
    source: Arc<str>,
    configuration: Option<Configuration>,
    diagnostics: Vec<Error>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ResolvedPackage {
    name: Option<String>,
    version: Option<String>,
}

impl ResolvedPackage {
    fn matches_specifier(&self, specifier: &str) -> bool {
        self.name.as_deref().is_some_and(|name| {
            specifier == name
                || specifier
                    .strip_prefix(name)
                    .is_some_and(|subpath| subpath.starts_with('/'))
        })
    }
}

#[derive(Clone, Debug)]
struct ConfigurationReference {
    from: Utf8PathBuf,
}

#[derive(Clone, Debug)]
struct PackageResolution {
    file_path: Utf8PathBuf,
    package: Option<ResolvedPackage>,
    reference: ConfigurationReference,
}

impl PackageResolution {
    fn version(&self) -> Option<&str> {
        self.package
            .as_ref()
            .and_then(|package| package.version.as_deref())
    }
}

impl From<&ResolvedExtendedConfiguration> for PackageResolution {
    fn from(resolved: &ResolvedExtendedConfiguration) -> Self {
        Self {
            file_path: resolved.file_path.clone(),
            package: resolved.package.clone(),
            reference: resolved.reference.clone(),
        }
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
    message(
        message("Biome resolved multiple versions of "<Emphasis>{self.specifier}</Emphasis>"."),
        description = "Biome resolved multiple versions of {specifier}."
    ),
    advice = "This is an error for Biome because it doesn't know which version to use. Extended configurations are applied in order, and a fixed configuration could be overridden by one that is bugged."
)]
struct MultipleExtendedConfigurationVersions {
    specifier: String,
    #[location(resource)]
    path: String,
    #[advice]
    advice: MultipleExtendedConfigurationVersionsAdvice,
}

#[derive(Debug)]
struct MultipleExtendedConfigurationVersionsAdvice {
    first: PackageResolution,
    conflicting: PackageResolution,
}

impl Advices for MultipleExtendedConfigurationVersionsAdvice {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        self.first
            .record_resolution(visitor, "The first reference")?;
        self.conflicting
            .record_resolution(visitor, "The conflicting reference")?;
        visitor.record_log(
            LogCategory::Info,
            &markup! { "Biome can't determine which configuration should be used." },
        )
    }
}

impl PackageResolution {
    fn record_resolution(&self, visitor: &mut dyn Visit, label: &str) -> std::io::Result<()> {
        let origin = self.reference.origin();
        let file_path = display_path(&self.file_path);
        match &self.package {
            Some(ResolvedPackage {
                name: Some(name),
                version: Some(version),
            }) => visitor.record_log(
                LogCategory::Info,
                &markup! {
                    {label}" from "{origin}" resolves to "
                    <Emphasis>{file_path}</Emphasis>" from "
                    <Emphasis>{name}"@"{version}</Emphasis>"."
                },
            ),
            Some(ResolvedPackage {
                version: Some(version),
                ..
            }) => visitor.record_log(
                LogCategory::Info,
                &markup! {
                    {label}" from "{origin}" resolves to "
                    <Emphasis>{file_path}</Emphasis>" from package version "
                    <Emphasis>{version}</Emphasis>"."
                },
            ),
            _ => visitor.record_log(
                LogCategory::Info,
                &markup! {
                    {label}" from "{origin}" resolves to "
                    <Emphasis>{file_path}</Emphasis>"."
                },
            ),
        }
    }
}

impl ConfigurationReference {
    fn origin(&self) -> Cow<'_, str> {
        display_path(&self.from)
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "configuration",
    severity = Error,
    message(
        message("The configuration extends chain exceeds the limit of "<Emphasis>{MAX_EXTENDS_DEPTH}</Emphasis>" levels."),
        description = "The configuration extends chain exceeds the limit of 10 levels."
    ),
    advice = "Reduce the number of nested extended configurations."
)]
struct ExtendsDepthLimit {
    #[location(resource)]
    path: String,
    #[advice]
    specifier: ExtendsDepthSpecifier,
}

#[derive(Debug)]
struct ExtendsDepthSpecifier(String);

impl From<String> for ExtendsDepthSpecifier {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Advices for ExtendsDepthSpecifier {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        visitor.record_log(
            LogCategory::Info,
            &markup! {
                "The configuration at the limit extends "<Emphasis>{self.0.as_str()}</Emphasis>"."
            },
        )
    }
}

/// Use this type to determine what kind of [ScanKind] needs to be used based
/// on the current configuration
pub struct ProjectScanComputer<'a> {
    requires_project_scan: bool,
    requires_types: bool,
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
            requires_types: false,
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
                }
                if domain == &RuleDomain::Types && value != &RuleDomainValue::None {
                    self.requires_types = true;
                    self.requires_project_scan = true;
                    // requiring types is of higher order of project, so we can bail
                    break;
                }
            }
        }

        #[cfg(feature = "lang_graphql")]
        biome_graphql_analyze::visit_registry(&mut self);
        #[cfg(feature = "lang_css")]
        biome_css_analyze::visit_registry(&mut self);
        biome_json_analyze::visit_registry(&mut self);
        #[cfg(feature = "lang_js")]
        biome_js_analyze::visit_registry(&mut self);
        #[cfg(feature = "lang_html")]
        biome_html_analyze::visit_registry(&mut self);

        if self.requires_types {
            ScanKind::TypeAware
        } else if self.requires_project_scan {
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

    #[inline]
    fn check_rule<R, L>(&mut self)
    where
        L: Language,
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        self.check_rule_name(<R::Group as RuleGroup>::NAME, &R::METADATA);
    }

    fn check_rule_name(&mut self, group_name: &'static str, metadata: &RuleMetadata) {
        let filter = RuleFilter::Rule(group_name, metadata.name);

        if !self.only.is_empty() {
            for selector in self.only.iter() {
                if selector.match_rule_name(group_name, metadata.name) {
                    let domains = metadata.domains;
                    self.requires_project_scan |= domains.contains(&RuleDomain::Project);
                    self.requires_types |= domains.contains(&RuleDomain::Types);
                    break;
                }
            }
        } else if !self
            .skip
            .iter()
            .any(|s| s.match_rule_name(group_name, metadata.name))
            && self.enabled_rules.contains(&filter)
        {
            let domains = metadata.domains;
            self.requires_project_scan |= domains.contains(&RuleDomain::Project);
            self.requires_types |= domains.contains(&RuleDomain::Types);
        }
    }
}

#[cfg(feature = "lang_js")]
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

#[cfg(feature = "lang_css")]
impl RegistryVisitor<CssLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, CssLanguage>();
    }
}
#[cfg(feature = "lang_graphql")]
impl RegistryVisitor<GraphqlLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, GraphqlLanguage>();
    }
}

#[cfg(feature = "lang_html")]
impl RegistryVisitor<HtmlLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, HtmlLanguage>();
    }
}

#[cfg(feature = "lang_md")]
impl RegistryVisitor<MarkdownLanguage> for ProjectScanComputer<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = MarkdownLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, MarkdownLanguage>();
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

    #[test]
    fn should_return_type_aware_if_type_aware_domain_is_enabled() {
        let mut domains = FxHashMap::default();
        domains.insert(RuleDomain::Types, RuleDomainValue::Recommended);

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                domains: Some(RuleDomains(domains)),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration).compute(),
            ScanKind::TypeAware
        );
    }

    #[test]
    fn should_return_type_aware_if_type_aware_domain_selector() {
        let configuration = Configuration::default();

        assert_eq!(
            ProjectScanComputer::new(&configuration)
                .with_rule_selectors(&[], &[DomainSelector("types").into()])
                .compute(),
            ScanKind::TypeAware
        );
    }

    #[test]
    fn should_return_type_aware_when_both_type_aware_and_project_enabled() {
        let mut domains = FxHashMap::default();
        domains.insert(RuleDomain::Project, RuleDomainValue::Recommended);
        domains.insert(RuleDomain::Types, RuleDomainValue::Recommended);

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                domains: Some(RuleDomains(domains)),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            ProjectScanComputer::new(&configuration).compute(),
            ScanKind::TypeAware
        );
    }

    #[test]
    fn should_not_return_type_aware_if_non_type_aware_domain() {
        let configuration = Configuration::default();

        assert_eq!(
            ProjectScanComputer::new(&configuration)
                .with_rule_selectors(&[], &[DomainSelector("react").into()])
                .compute(),
            ScanKind::NoScanner
        );
    }
}

#[cfg(test)]
mod configuration_harness {
    use biome_analyze::{GroupCategory, Queryable, RegistryVisitor, Rule, RuleCategory, RuleGroup};
    use biome_configuration::generated::linter_options_check::config_side_rule_options_types;
    use biome_css_syntax::CssLanguage;
    use biome_graphql_syntax::GraphqlLanguage;
    use biome_html_syntax::HtmlLanguage;
    use biome_js_syntax::JsLanguage;
    use biome_json_syntax::JsonLanguage;
    use std::any::TypeId;
    use std::collections::HashMap;

    /// Collects `TypeId::of::<R::Options>()` for every rule via the registry visitor.
    /// This is the "rule side" type — what the rule declares as `type Options`.
    struct RuleSideOptionsVisitor {
        types: HashMap<(&'static str, &'static str), TypeId>,
    }

    impl RuleSideOptionsVisitor {
        fn collect_rule<R, L>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
        {
            let category = <R::Group as RuleGroup>::Category::CATEGORY;
            if !matches!(category, RuleCategory::Lint) {
                return;
            }
            self.types.insert(
                (<R::Group as RuleGroup>::NAME, R::METADATA.name),
                TypeId::of::<R::Options>(),
            );
        }
    }

    impl RegistryVisitor<JsLanguage> for RuleSideOptionsVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>>
                + 'static,
        {
            self.collect_rule::<R, JsLanguage>();
        }
    }

    impl RegistryVisitor<JsonLanguage> for RuleSideOptionsVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
                + 'static,
        {
            self.collect_rule::<R, JsonLanguage>();
        }
    }

    impl RegistryVisitor<CssLanguage> for RuleSideOptionsVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
                + 'static,
        {
            self.collect_rule::<R, CssLanguage>();
        }
    }

    impl RegistryVisitor<GraphqlLanguage> for RuleSideOptionsVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
                + 'static,
        {
            self.collect_rule::<R, GraphqlLanguage>();
        }
    }

    impl RegistryVisitor<HtmlLanguage> for RuleSideOptionsVisitor {
        fn record_rule<R>(&mut self)
        where
            R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
                + 'static,
        {
            self.collect_rule::<R, HtmlLanguage>();
        }
    }

    /// Verifies that every lint rule's `type Options` matches the canonical options
    /// type derived from the rule name (`biome_rule_options::{snake_name}::{Name}Options`).
    ///
    /// This catches copy-paste bugs where a rule accidentally uses another rule's
    /// options type (e.g. `type Options = SomeOtherRuleOptions`). The configuration
    /// layer always constructs `RuleOptions` using the canonical type, so a mismatch
    /// causes a `TypeId` divergence that triggers a panic at runtime.
    #[test]
    fn rule_options_match_config_types() {
        let config_side = config_side_rule_options_types();

        let mut visitor = RuleSideOptionsVisitor {
            types: HashMap::new(),
        };
        biome_js_analyze::visit_registry(&mut visitor);
        biome_json_analyze::visit_registry(&mut visitor);
        biome_css_analyze::visit_registry(&mut visitor);
        biome_graphql_analyze::visit_registry(&mut visitor);
        biome_html_analyze::visit_registry(&mut visitor);

        let mut mismatches = Vec::new();
        for (group, rule, config_type_id) in &config_side {
            if let Some(rule_type_id) = visitor.types.get(&(*group, *rule))
                && config_type_id != rule_type_id
            {
                mismatches.push(format!(
                    "  {group}/{rule}: rule declares a different Options type than \
                         biome_rule_options::{module}::{name}Options",
                    module = biome_string_case::Case::Snake.convert(rule),
                    name = {
                        let mut c = rule.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    },
                ));
            }
        }

        if !mismatches.is_empty() {
            panic!(
                "Rule options type mismatches detected:\n{}\n\n\
                 Each rule's `type Options` must match the canonical options type \
                 generated from its name. Check for copy-paste errors.",
                mismatches.join("\n")
            );
        }
    }
}

#[cfg(test)]
mod test {
    #[cfg(unix)]
    use super::ConfigurationExtendsLoader;
    use crate::{WorkspaceError, configuration::load_configuration};
    use biome_configuration::{
        BiomeDiagnostic, ConfigurationPathHint, diagnostics::ConfigurationDiagnostic,
    };
    use biome_diagnostics::Severity;
    use biome_fs::MemoryFileSystem;
    #[cfg(unix)]
    use biome_fs::TemporaryFs;
    use camino::Utf8PathBuf;
    #[cfg(unix)]
    use std::os::unix::fs::symlink;

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
                        .resolved_configuration()
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

    #[test]
    fn explicit_package_exports_block_manifest_fallback() {
        for target in ["null", r#""./missing.json""#] {
            let fs = MemoryFileSystem::default();
            fs.insert(
                Utf8PathBuf::from("/project/biome.json"),
                r#"{ "extends": ["package/configs/recommended"] }"#,
            );
            fs.insert(
                Utf8PathBuf::from("/project/node_modules/package/package.json"),
                format!(
                    r#"{{
                        "name": "package",
                        "exports": {{ "./configs/recommended": {target} }}
                    }}"#
                ),
            );
            fs.insert(
                Utf8PathBuf::from("/project/node_modules/package/biome-manifest.json"),
                r#"{
                    "version": 1,
                    "configs": [{ "recommended": "./recommended.json" }]
                }"#,
            );
            fs.insert(
                Utf8PathBuf::from("/project/node_modules/package/recommended.json"),
                r#"{ "formatter": { "lineWidth": 100 } }"#,
            );

            let result = load_configuration(
                &fs,
                ConfigurationPathHint::FromUser(Utf8PathBuf::from("/project/biome.json")),
            );
            assert!(
                matches!(
                    result,
                    Err(WorkspaceError::Configuration(
                        ConfigurationDiagnostic::Biome(BiomeDiagnostic::CantResolve(_))
                    ))
                ),
                "export target {target} should block manifest fallback"
            );
        }
    }

    #[test]
    fn nested_manifest_config_selections_are_rejected() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["wrapper/configs/source/recommended"] }"#,
        );

        let result = load_configuration(
            &fs,
            ConfigurationPathHint::FromUser(Utf8PathBuf::from("/project/biome.json")),
        );
        assert!(matches!(
            result,
            Err(WorkspaceError::Configuration(
                ConfigurationDiagnostic::Biome(BiomeDiagnostic::CantResolve(_))
            ))
        ));
    }

    #[test]
    fn reexported_manifest_config_names_must_be_unique() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["wrapper/configs/recommended"] }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/node_modules/wrapper/package.json"),
            r#"{ "name": "wrapper" }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/node_modules/wrapper/biome-manifest.json"),
            r#"{
                "version": 1,
                "configs": [
                    "source-a/configs/recommended",
                    "source-b/configs/recommended"
                ]
            }"#,
        );
        for source in ["source-a", "source-b"] {
            fs.insert(
                format!("/project/node_modules/{source}/package.json").into(),
                format!(r#"{{ "name": "{source}" }}"#),
            );
            fs.insert(
                format!("/project/node_modules/{source}/biome-manifest.json").into(),
                r#"{
                    "version": 1,
                    "configs": [{ "recommended": "./recommended.json" }]
                }"#,
            );
            fs.insert(
                format!("/project/node_modules/{source}/recommended.json").into(),
                "{}",
            );
        }

        let result = load_configuration(
            &fs,
            ConfigurationPathHint::FromUser(Utf8PathBuf::from("/project/biome.json")),
        );
        assert!(matches!(
            result,
            Err(WorkspaceError::Configuration(
                ConfigurationDiagnostic::Biome(BiomeDiagnostic::CantResolve(_))
            ))
        ));
    }

    #[cfg(unix)]
    #[test]
    fn manifest_config_paths_reject_symbolic_links() {
        let mut fs = TemporaryFs::new("biome_service_manifest_config_symlinks");
        let outside_config = fs.create_file("outside/base.json", "{}");
        fs.create_file("directory-link/biome-manifest.json", "{}");
        fs.create_file("file-link/biome-manifest.json", "{}");

        symlink(
            outside_config.parent().unwrap(),
            fs.working_directory.join("directory-link/configs"),
        )
        .unwrap();
        std::fs::create_dir_all(fs.working_directory.join("file-link/configs")).unwrap();
        symlink(
            &outside_config,
            fs.working_directory.join("file-link/configs/base.json"),
        )
        .unwrap();

        let os = fs.create_os();
        for manifest_dir in ["directory-link", "file-link"] {
            ConfigurationExtendsLoader::resolve_manifest_config_path(
                &os,
                &fs.working_directory.join(manifest_dir),
                "configs/base.json",
            )
            .expect_err("manifest configuration paths should not contain symbolic links");
        }
    }

    #[test]
    fn should_preserve_unmerged_configuration_sources() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["./first.json", "./second.json"] }"#.to_string(),
        );
        fs.insert(
            Utf8PathBuf::from("/project/first.json"),
            r#"{ "formatter": { "lineWidth": 100 } }"#.to_string(),
        );
        fs.insert(
            Utf8PathBuf::from("/project/second.json"),
            r#"{ "formatter": { "indentWidth": 4 } }"#.to_string(),
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromUser(Utf8PathBuf::from("/project/biome.json")),
        )
        .expect("valid configuration");

        assert_eq!(
            loaded
                .source
                .extended_configurations
                .as_slice()
                .iter()
                .filter_map(|extended| extended.source.file_path.as_deref())
                .collect::<Vec<_>>(),
            [
                Utf8PathBuf::from("/project/first.json"),
                Utf8PathBuf::from("/project/second.json"),
            ]
        );
        assert_eq!(
            loaded
                .source
                .root
                .as_ref()
                .and_then(|root| root.file_source.as_deref()),
            Some(r#"{ "extends": ["./first.json", "./second.json"] }"#)
        );
        assert_eq!(
            loaded
                .source
                .extended_configurations
                .as_slice()
                .iter()
                .map(|extended| (
                    extended.specifier.as_deref(),
                    extended.source.file_source.as_deref()
                ))
                .collect::<Vec<_>>(),
            [
                (
                    Some("./first.json"),
                    Some(r#"{ "formatter": { "lineWidth": 100 } }"#)
                ),
                (
                    Some("./second.json"),
                    Some(r#"{ "formatter": { "indentWidth": 4 } }"#)
                ),
            ]
        );
        assert_eq!(
            loaded
                .source
                .root
                .as_ref()
                .and_then(|root| root.configuration.as_ref())
                .and_then(|configuration| configuration.formatter.as_ref()),
            None
        );
        assert_eq!(
            loaded
                .resolved_configuration()
                .formatter
                .and_then(|formatter| formatter.line_width)
                .map(u16::from),
            Some(100)
        );
    }

    #[test]
    fn should_load_nested_extends_in_dependency_first_order() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["./first.json", "./sibling.json"] }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/first.json"),
            r#"{ "extends": ["./base.json"], "formatter": { "indentWidth": 4 } }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/base.json"),
            r#"{ "formatter": { "lineWidth": 90 } }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/sibling.json"),
            r#"{ "formatter": { "lineWidth": 100 } }"#,
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("valid configuration");

        assert_eq!(
            loaded
                .source
                .extended_configurations
                .as_slice()
                .iter()
                .filter_map(|extended| extended.source.file_path.as_deref())
                .collect::<Vec<_>>(),
            [
                Utf8PathBuf::from("/project/base.json"),
                Utf8PathBuf::from("/project/first.json"),
                Utf8PathBuf::from("/project/sibling.json"),
            ]
        );
        let formatter = loaded
            .resolved_configuration()
            .formatter
            .expect("formatter configuration");
        assert_eq!(formatter.line_width.map(u16::from), Some(100));
        assert_eq!(formatter.indent_width.map(|width| width.value()), Some(4));
        assert!(!loaded.has_errors());
    }

    #[test]
    fn should_load_ten_nested_extends_levels() {
        let fs = MemoryFileSystem::default();
        insert_extends_chain(&fs, 10);

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("valid configuration");

        assert_eq!(loaded.source.extended_configurations.as_slice().len(), 10);
        assert!(!loaded.has_errors());
    }

    #[test]
    fn should_reject_an_eleventh_nested_extends_level() {
        let fs = MemoryFileSystem::default();
        insert_extends_chain(&fs, 10);
        fs.insert(
            Utf8PathBuf::from("/project/level-10.json"),
            r#"{ "extends": ["./level-11.json"] }"#,
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("configuration loading result");

        assert_eq!(loaded.source.extended_configurations.as_slice().len(), 10);
        assert!(loaded.has_errors());
        assert_eq!(
            loaded
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
                .count(),
            1
        );
    }

    #[test]
    fn should_load_a_repeated_identical_extended_configuration() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["./first.json", "./second.json"] }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/first.json"),
            r#"{ "extends": ["./shared.json"] }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/second.json"),
            r#"{ "extends": ["./shared.json"] }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/shared.json"),
            r#"{ "formatter": { "lineWidth": 90 } }"#,
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("valid configuration");

        assert_eq!(
            loaded
                .source
                .extended_configurations
                .as_slice()
                .iter()
                .filter_map(|extended| extended.source.file_path.as_deref())
                .collect::<Vec<_>>(),
            [
                Utf8PathBuf::from("/project/shared.json"),
                Utf8PathBuf::from("/project/first.json"),
                Utf8PathBuf::from("/project/shared.json"),
                Utf8PathBuf::from("/project/second.json"),
            ]
        );
        assert!(loaded.diagnostics.is_empty());
        assert!(!loaded.has_errors());
    }

    #[test]
    fn should_reject_an_extends_cycle_at_the_depth_limit() {
        let fs = MemoryFileSystem::default();
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["./shared.json"] }"#,
        );
        fs.insert(
            Utf8PathBuf::from("/project/shared.json"),
            r#"{ "extends": ["./biome.json"], "formatter": { "lineWidth": 90 } }"#,
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("valid configuration");

        assert_eq!(loaded.source.extended_configurations.as_slice().len(), 10);
        assert_eq!(
            loaded
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
                .count(),
            1
        );
        assert!(loaded.has_errors());
    }

    #[test]
    fn should_reject_one_package_specifier_resolving_to_different_versions() {
        let fs = MemoryFileSystem::default();
        insert_nested_package_graph(
            &fs,
            ("1.0.0", r#"{ "formatter": { "lineWidth": 90 } }"#),
            ("2.0.0", r#"{ "formatter": { "lineWidth": 100 } }"#),
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("configuration loading result");

        assert!(loaded.has_errors());
        assert_eq!(
            loaded
                .diagnostics
                .iter()
                .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
                .count(),
            1
        );
        assert_eq!(
            loaded
                .source
                .extended_configurations
                .as_slice()
                .iter()
                .filter_map(|extended| extended.source.file_path.as_deref())
                .collect::<Vec<_>>(),
            [
                Utf8PathBuf::from(
                    "/project/node_modules/package-a/node_modules/shared-config/biome.json"
                ),
                Utf8PathBuf::from("/project/node_modules/package-a/biome.json"),
                Utf8PathBuf::from("/project/node_modules/package-b/biome.json"),
            ]
        );
    }

    #[test]
    fn should_load_one_package_version_from_different_paths() {
        let fs = MemoryFileSystem::default();
        insert_nested_package_graph(
            &fs,
            ("1.0.0", r#"{ "formatter": { "lineWidth": 90 } }"#),
            ("1.0.0", r#"{ "formatter": { "lineWidth": 100 } }"#),
        );

        let loaded = load_configuration(
            &fs,
            ConfigurationPathHint::FromWorkspace(Utf8PathBuf::from("/project")),
        )
        .expect("valid configuration");

        assert!(!loaded.has_errors());
        assert_eq!(loaded.source.extended_configurations.as_slice().len(), 4);
        assert_eq!(
            loaded
                .resolved_configuration()
                .formatter
                .and_then(|formatter| formatter.line_width)
                .map(u16::from),
            Some(100)
        );
    }

    fn insert_extends_chain(fs: &MemoryFileSystem, levels: usize) {
        let root_extends = if levels == 0 {
            String::new()
        } else {
            r#""extends": ["./level-1.json"]"#.to_string()
        };
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            format!("{{ {root_extends} }}"),
        );
        for level in 1..=levels {
            let source = if level == levels {
                r#"{ "formatter": { "lineWidth": 90 } }"#.to_string()
            } else {
                format!(r#"{{ "extends": ["./level-{}.json"] }}"#, level + 1)
            };
            fs.insert(
                Utf8PathBuf::from(format!("/project/level-{level}.json")),
                source,
            );
        }
    }

    fn insert_configuration_package(
        fs: &MemoryFileSystem,
        directory: &str,
        name: &str,
        version: &str,
        configuration: &str,
    ) {
        fs.insert(
            Utf8PathBuf::from(format!("{directory}/package.json")),
            format!(r#"{{ "name": "{name}", "version": "{version}", "main": "biome.json" }}"#),
        );
        fs.insert(
            Utf8PathBuf::from(format!("{directory}/biome.json")),
            configuration,
        );
    }

    fn insert_nested_package_graph(
        fs: &MemoryFileSystem,
        package_a_configuration: (&str, &str),
        package_b_configuration: (&str, &str),
    ) {
        fs.insert(
            Utf8PathBuf::from("/project/biome.json"),
            r#"{ "extends": ["package-a", "package-b"] }"#,
        );
        for parent in ["package-a", "package-b"] {
            insert_configuration_package(
                fs,
                &format!("/project/node_modules/{parent}"),
                parent,
                "1.0.0",
                r#"{ "extends": ["shared-config"] }"#,
            );
        }
        for (parent, (version, configuration)) in [
            ("package-a", package_a_configuration),
            ("package-b", package_b_configuration),
        ] {
            insert_configuration_package(
                fs,
                &format!("/project/node_modules/{parent}/node_modules/shared-config"),
                "shared-config",
                version,
                configuration,
            );
        }
    }
}
