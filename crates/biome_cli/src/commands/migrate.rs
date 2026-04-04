use crate::cli_options::CliOptions;
use crate::commands::{FixFileModeOptions, MigrateSubCommand, check_fix_incompatible_arguments};
use crate::diagnostics::MigrationDiagnostic;
use crate::runner::ConfiguredWorkspace;
use crate::runner::execution::{AnalyzerSelectors, Execution};
use crate::runner::impls::commands::custom_execution::CustomExecutionCmd;
use crate::{CliDiagnostic, CliSession};
use biome_configuration::Configuration;
use biome_console::{Console, ConsoleExt, MarkupBuf, markup};
use biome_diagnostics::{Category, category};
use biome_fs::FileSystem;
use biome_service::workspace::{
    FeatureName, FeaturesBuilder, FeaturesSupported, ScanKind, SupportKind,
};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::time::Duration;

pub(crate) struct MigrateCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) sub_command: Option<MigrateSubCommand>,
    pub(crate) configuration_file_path: Option<Utf8PathBuf>,
    pub(crate) configuration_directory_path: Option<Utf8PathBuf>,
}

pub(crate) struct MigrateExecution {
    write: bool,
}

impl Execution for MigrateExecution {
    fn wanted_features(&self) -> FeatureName {
        FeaturesBuilder::new().build()
    }

    fn not_requested_features(&self) -> FeatureName {
        FeaturesBuilder::new().build()
    }

    fn can_handle(&self, _: FeaturesSupported) -> bool {
        true
    }

    fn is_vcs_targeted(&self) -> bool {
        false
    }

    fn supports_kind(&self, _: &FeaturesSupported) -> Option<SupportKind> {
        None
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        None
    }

    fn as_diagnostic_category(&self) -> &'static Category {
        category!("migrate")
    }

    fn requires_write_access(&self) -> bool {
        self.write
    }

    fn analyzer_selectors(&self) -> AnalyzerSelectors {
        AnalyzerSelectors::default()
    }
    fn summary_phrase(&self, _files: usize, duration: &Duration) -> MarkupBuf {
        if self.requires_write_access() {
            markup! {
              "Migrated your configuration file in "{duration}"."
            }
            .to_owned()
        } else {
            markup! {
                "Checked your configuration file in "{duration}"."
            }
            .to_owned()
        }
    }
}

impl CustomExecutionCmd for MigrateCommandPayload {
    fn command_name(&self) -> &'static str {
        "migrate"
    }

    fn minimal_scan_kind(&self) -> Option<ScanKind> {
        Some(ScanKind::KnownFiles)
    }

    fn should_write(&self) -> bool {
        self.write || self.fix
    }

    fn get_execution(
        &self,
        _cli_options: &CliOptions,
        console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        if self.configuration_file_path.is_some() {
            Ok(Box::new(MigrateExecution {
                write: self.should_write(),
            }))
        } else {
            console.log(markup! {
            <Info>"If this project has not yet been set up with Biome yet, please follow the "<Hyperlink href="https://biomejs.dev/guides/getting-started/">"Getting Started guide"</Hyperlink>" first."</Info>
        });
            Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
                reason: "Biome couldn't find the Biome configuration file.".to_string(),
            }))
        }
    }

    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        check_fix_incompatible_arguments(FixFileModeOptions {
            write: self.write,
            fix: self.fix,
            unsafe_: false,
            suppress: false,
            suppression_reason: None,
        })
    }

    fn should_validate_configuration_diagnostics(&self) -> bool {
        false
    }

    fn execute_without_crawling(
        &mut self,
        session: CliSession,
        configured_workspace: ConfiguredWorkspace,
    ) -> Result<(), CliDiagnostic> {
        let ConfiguredWorkspace {
            execution: _,
            project_key,
            paths: _,
            configuration_files,
            duration: _,
        } = configured_workspace;

        let payload = crate::execute::migrate::MigratePayload {
            session,
            project_key,
            write: self.should_write(),
            // SAFETY: checked during get_execution
            configuration_file_path: self.configuration_file_path.clone().unwrap(),
            sub_command: self.sub_command.clone(),
            nested_configuration_files: configuration_files,
        };
        crate::execute::migrate::run(payload)
    }

    fn merge_configuration(
        &mut self,
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        loaded_file: Option<Utf8PathBuf>,
        _fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        self.configuration_file_path = loaded_file;
        self.configuration_directory_path = loaded_directory;
        Ok(loaded_configuration)
    }
}
