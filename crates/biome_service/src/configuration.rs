use crate::settings::WorkspaceSettings;
use crate::{DynRef, WorkspaceError, VERSION};
use biome_analyze::AnalyzerRules;
use biome_configuration::diagnostics::CantLoadExtendFile;
use biome_configuration::{
    push_to_analyzer_rules, ConfigurationBasePath, ConfigurationDiagnostic, ConfigurationPayload,
    PartialConfiguration,
};
use biome_console::markup;
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::{Deserialized, Merge};
use biome_diagnostics::{DiagnosticExt, Error, Severity};
use biome_fs::{AutoSearchResult, FileSystem, OpenOptions};
use biome_js_analyze::metadata;
use biome_json_formatter::context::JsonFormatOptions;
use biome_json_parser::{parse_json, JsonParserOptions};
use std::io::ErrorKind;
use std::iter::FusedIterator;
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
    pub fn new(errors: &'a [Error]) -> Self {
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
            configuration_directory_path,
            configuration_file_path,
            deserialized,
        } = value;
        let (partial_configuration, mut diagnostics) = deserialized.consume();

        Ok(Self {
            configuration: match partial_configuration {
                Some(mut partial_configuration) => {
                    apply_extends(
                        &mut partial_configuration,
                        fs,
                        &configuration_file_path,
                        &configuration_directory_path,
                        &mut diagnostics,
                    )?;
                    migrate_deprecated_fields(&mut partial_configuration);
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
            directory_path: Some(configuration_directory_path),
            file_path: Some(configuration_file_path),
        })
    }
}

/// Load the partial configuration for this session of the CLI.
pub fn load_configuration(
    fs: &DynRef<'_, dyn FileSystem>,
    config_path: ConfigurationBasePath,
) -> Result<LoadedConfiguration, WorkspaceError> {
    let config = load_config(fs, config_path)?;
    LoadedConfiguration::try_from_payload(config, fs)
}

/// - [Result]: if an error occurred while loading the configuration file.
/// - [Option]: sometimes not having a configuration file should not be an error, so we need this type.
/// - [ConfigurationPayload]: The result of the operation
pub type LoadConfig = Result<Option<ConfigurationPayload>, WorkspaceError>;

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
        let deserialized = deserialize_from_json_str::<PartialConfiguration>(
            &content,
            JsonParserOptions::default(),
            "",
        );
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
    mut configuration: PartialConfiguration,
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
        configuration.schema = Some(format!("https://biomejs.dev/schemas/{VERSION}/schema.json"));
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

/// Mutates the configuration so that any fields that have not been configured explicitly are
/// filled in with their values from configs listed in the `extends` field.
///
/// The `extends` configs are applied from left to right.
///
/// If a configuration can't be resolved from the file system, the operation will fail.
fn apply_extends(
    partial_configuration: &mut PartialConfiguration,
    fs: &DynRef<'_, dyn FileSystem>,
    file_path: &Path,
    directory_path: &Path,
    diagnostics: &mut Vec<Error>,
) -> Result<(), WorkspaceError> {
    let deserialized = deserialize_extends(partial_configuration, fs, directory_path)?;
    let (configurations, errors): (Vec<_>, Vec<_>) = deserialized
        .into_iter()
        .map(|d| d.consume())
        .map(|(config, diagnostics)| (config.unwrap_or_default(), diagnostics))
        .unzip();

    let extended_configuration =
        configurations
            .into_iter()
            .reduce(|mut previous_configuration, current_configuration| {
                previous_configuration.merge_with(current_configuration);
                previous_configuration
            });
    if let Some(mut extended_configuration) = extended_configuration {
        // We swap them to avoid having to clone `self.configuration` to merge it.
        std::mem::swap(partial_configuration, &mut extended_configuration);
        partial_configuration.merge_with(extended_configuration)
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
    partial_configuration: &mut PartialConfiguration,
    fs: &DynRef<'_, dyn FileSystem>,
    directory_path: &Path,
) -> Result<Vec<Deserialized<PartialConfiguration>>, WorkspaceError> {
    let Some(extends) = &partial_configuration.extends else {
        return Ok(Vec::new());
    };

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
        let deserialized = deserialize_from_json_str::<PartialConfiguration>(
            content.as_str(),
            JsonParserOptions::default(),
            "",
        );
        deserialized_configurations.push(deserialized)
    }
    Ok(deserialized_configurations)
}

/// Checks for the presence of deprecated fields and updates the
/// configuration to apply them to the new schema.
fn migrate_deprecated_fields(partial_configuration: &mut PartialConfiguration) {
    // TODO: remove in biome 2.0
    if let Some(formatter) = partial_configuration
        .css
        .as_mut()
        .and_then(|css| css.formatter.as_mut())
    {
        if formatter.indent_size.is_some() && formatter.indent_width.is_none() {
            formatter.indent_width = formatter.indent_size;
        }
    }

    // TODO: remove in biome 2.0
    if let Some(formatter) = partial_configuration.formatter.as_mut() {
        if formatter.indent_size.is_some() && formatter.indent_width.is_none() {
            formatter.indent_width = formatter.indent_size;
        }
    }

    // TODO: remove in biome 2.0
    if let Some(formatter) = partial_configuration
        .javascript
        .as_mut()
        .and_then(|js| js.formatter.as_mut())
    {
        if formatter.indent_size.is_some() && formatter.indent_width.is_none() {
            formatter.indent_width = formatter.indent_size;
        }
    }

    // TODO: remove in biome 2.0
    if let Some(formatter) = partial_configuration
        .json
        .as_mut()
        .and_then(|json| json.formatter.as_mut())
    {
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
pub fn retrieve_gitignore_matches(
    partial_configuration: &PartialConfiguration,
    file_system: &DynRef<'_, dyn FileSystem>,
    vcs_base_path: Option<&Path>,
) -> Result<(Option<PathBuf>, Vec<String>), WorkspaceError> {
    let Some(vcs) = &partial_configuration.vcs else {
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
                    .auto_search(vcs_base_path, client_kind.ignore_file(), false)
                    .map_err(WorkspaceError::from)?;

                if let Some(result) = result {
                    return Ok((
                        Some(result.directory_path),
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
