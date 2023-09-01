use crate::cli_options::CliOptions;
use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, Console, ConsoleExt};
use rome_deserialize::json::deserialize_from_json_str;
use rome_deserialize::Deserialized;
use rome_diagnostics::{DiagnosticExt, Error, PrintDiagnostic};
use rome_fs::{FileSystem, OpenOptions};
use rome_json_parser::JsonParserOptions;
use rome_service::configuration::diagnostics::CantLoadExtendFile;
use rome_service::configuration::ConfigurationPayload;
use rome_service::{
    load_config, Configuration, ConfigurationBasePath, ConfigurationDiagnostic, DynRef, MergeWith,
    WorkspaceError,
};
use std::path::PathBuf;

#[derive(Default)]
pub struct LoadedConfiguration {
    pub(crate) directory_path: Option<PathBuf>,
    pub(crate) file_path: Option<PathBuf>,
    pub(crate) configuration: Configuration,
    pub(crate) diagnostics: Vec<Error>,
}

impl LoadedConfiguration {
    /// It updates the loaded configuration by resolving the `extends` field.
    ///
    /// If a configuration can't be resolved from the file system, the operation will fail.
    pub fn apply_extends(&mut self, fs: &DynRef<dyn FileSystem>) -> Result<(), WorkspaceError> {
        let deserialized = self.deserialize_extends(fs)?;
        let (configurations, errors): (Vec<_>, Vec<_>) =
            deserialized.into_iter().map(|d| d.consume()).unzip();
        for c in configurations {
            self.configuration.merge_with(c);
        }
        self.diagnostics
            .extend(errors.into_iter().flatten().collect::<Vec<_>>());

        Ok(())
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

    pub fn or_diagnostic(
        self,
        console: &mut dyn Console,
        verbose: bool,
    ) -> Result<Self, CliDiagnostic> {
        if !self.diagnostics.is_empty() {
            for diagnostic in self.diagnostics {
                let diagnostic = if let Some(file_path) = &self.file_path {
                    diagnostic.with_file_path(file_path.display().to_string())
                } else {
                    diagnostic
                };
                console.error(markup! {
					{if verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
            	})
            }
            return Err(CliDiagnostic::workspace_error(
                WorkspaceError::Configuration(ConfigurationDiagnostic::invalid_configuration(
                    "Biome exited because the configuration resulted in errors. Please fix them.",
                )),
            ));
        }

        Ok(self)
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
                configuration,
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
    let mut loaded_configuration = LoadedConfiguration::from(config);

    loaded_configuration.apply_extends(fs)?;
    Ok(loaded_configuration)
}
