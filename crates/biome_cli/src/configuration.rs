use crate::cli_options::CliOptions;
use crate::diagnostics::DeprecatedConfigurationFile;
use crate::{CliDiagnostic, CliSession};
use biome_console::{markup, Console, ConsoleExt};
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Deserialized;
use biome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic, Severity};
use biome_fs::{FileSystem, OpenOptions};
use biome_json_parser::JsonParserOptions;
use biome_service::configuration::diagnostics::CantLoadExtendFile;
use biome_service::configuration::ConfigurationPayload;
use biome_service::{
    load_config, Configuration, ConfigurationBasePath, ConfigurationDiagnostic, DynRef, MergeWith,
    WorkspaceError,
};
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct LoadedConfiguration {
    pub(crate) directory_path: Option<PathBuf>,
    pub(crate) file_path: Option<PathBuf>,
    pub(crate) configuration: Configuration,
    pub(crate) diagnostics: Vec<Error>,
}

impl LoadedConfiguration {
    /// Consumes itself to generate a new [LoadedConfiguration] where the new `configuration`
    /// is the result of its `extends` fields applied from left to right, and the last one element
    /// applied is itself.
    ///
    /// If a configuration can't be resolved from the file system, the operation will fail.
    pub fn apply_extends(mut self, fs: &DynRef<dyn FileSystem>) -> Result<Self, WorkspaceError> {
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
        self.diagnostics
            .extend(errors.into_iter().flatten().collect::<Vec<_>>());

        Ok(Self {
            configuration,
            diagnostics: self.diagnostics,
            file_path: self.file_path,
            directory_path: self.directory_path,
        })
    }

    fn deserialize_extends(
        &mut self,
        fs: &DynRef<dyn FileSystem>,
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
        for path in extends.index_set() {
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

    /// It re
    #[must_use]
    pub fn with_file_path(mut self) -> Self {
        self.diagnostics = self
            .diagnostics
            .into_iter()
            .map(|diagnostic| {
                if let Some(file_path) = &self.file_path {
                    diagnostic.with_file_path(file_path.display().to_string())
                } else {
                    diagnostic
                }
            })
            .collect::<Vec<_>>();
        self
    }

    /// It prints diagnostics to console if there are any, and return [Err] if any of them is an error
    pub fn check_for_errors(
        &self,
        console: &mut dyn Console,
        verbose: bool,
    ) -> Result<(), CliDiagnostic> {
        let hss_errors = self
            .diagnostics
            .iter()
            .any(|e| e.severity() == Severity::Error);

        if !self.diagnostics.is_empty() {
            for diagnostic in &self.diagnostics {
                console.error(markup! {
					{if verbose { PrintDiagnostic::verbose(diagnostic) } else { PrintDiagnostic::simple(diagnostic) }}
            	})
            }
        }
        if hss_errors {
            return Err(CliDiagnostic::workspace_error(
                WorkspaceError::Configuration(ConfigurationDiagnostic::invalid_configuration(
                    "Biome exited because the configuration resulted in errors. Please fix them.",
                )),
            ));
        }

        if let Some(file_path) = self
            .file_path
            .as_ref()
            .and_then(|f| f.file_name())
            .and_then(|f| f.to_str())
        {
            if file_path == "rome.json" {
                let diagnostic = DeprecatedConfigurationFile::new(file_path);
                console.error(markup! {
					{if verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
            	})
            }
        }

        Ok(())
    }
}

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
                diagnostics,
                directory_path: Some(configuration_directory_path),
                file_path: Some(configuration_file_path),
            }
        } else {
            LoadedConfiguration::default()
        }
    }
}

/// Load the configuration for this session of the CLI, merging the content of
/// the `biome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(
    session: &mut CliSession,
    cli_options: &CliOptions,
) -> Result<LoadedConfiguration, CliDiagnostic> {
    let base_path = match &cli_options.config_path {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(PathBuf::from(path)),
    };

    let fs = &session.app.fs;
    let config = load_config(fs, base_path)?;
    let loaded_configuration = LoadedConfiguration::from(config);
    Ok(loaded_configuration.apply_extends(fs)?)
}
