use crate::CliDiagnostic;
use crate::changed::get_changed_files;
use crate::cli_options::{CliOptions, CliReporter, CliReporterKind};
use crate::runner::execution::{AnalyzerSelectors, Execution, ExecutionEnvironment, VcsTargeted};
use crate::runner::impls::commands::traversal::{LoadEditorConfig, TraversalCommand};
use crate::runner::impls::executions::summary_verb::SummaryVerbExecution;
use crate::runner::impls::process_file::check::CheckProcessFile;
use biome_configuration::analyzer::AnalyzerSelector;
use biome_configuration::analyzer::LinterEnabled;
use biome_configuration::analyzer::assist::{AssistConfiguration, AssistEnabled};
use biome_configuration::css::CssParserConfiguration;
use biome_configuration::formatter::{FormatWithErrorsEnabled, FormatterEnabled};
use biome_configuration::json::JsonParserConfiguration;
use biome_configuration::{
    Configuration, CssConfiguration, FormatterConfiguration, JsonConfiguration, LinterConfiguration,
};
use biome_console::{Console, MarkupBuf};
use biome_deserialize::Merge;
use biome_diagnostics::{Category, category};
use biome_fs::FileSystem;
use biome_service::workspace::{
    FeatureKind, FeatureName, FeaturesBuilder, FeaturesSupported, ScanKind, SupportKind,
};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;
use std::time::Duration;

pub(crate) struct CiCommandPayload {
    pub(crate) formatter_enabled: Option<FormatterEnabled>,
    pub(crate) linter_enabled: Option<LinterEnabled>,
    pub(crate) assist_enabled: Option<AssistEnabled>,
    pub(crate) enforce_assist: bool,
    pub(crate) paths: Vec<OsString>,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
    pub(crate) format_with_errors: Option<FormatWithErrorsEnabled>,
    pub(crate) json_parser: Option<JsonParserConfiguration>,
    pub(crate) css_parser: Option<CssParserConfiguration>,
    pub(crate) only: Vec<AnalyzerSelector>,
    pub(crate) skip: Vec<AnalyzerSelector>,
}

struct CiExecution {
    /// Whether the CI is running in a specific environment, e.g. GitHub, GitLab, etc.
    environment: Option<ExecutionEnvironment>,
    /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
    vcs_targeted: VcsTargeted,
    /// Whether assist diagnostics should be promoted to error, and fail the CLI
    enforce_assist: bool,
    /// It skips parse errors
    skip_parse_errors: bool,
    /// Run only the given rule or group of rules.
    only: Vec<AnalyzerSelector>,
    /// Skip the given rule or group of rules.
    skip: Vec<AnalyzerSelector>,
}

impl Execution for CiExecution {
    fn wanted_features(&self) -> FeatureName {
        FeaturesBuilder::new().with_all().without_search().build()
    }

    fn not_requested_features(&self) -> FeatureName {
        FeaturesBuilder::new().with_search().build()
    }

    fn can_handle(&self, features: FeaturesSupported) -> bool {
        features.supports_lint() || features.supports_assist() || features.supports_format()
    }

    fn is_vcs_targeted(&self) -> bool {
        self.vcs_targeted.changed || self.vcs_targeted.staged
    }

    fn supports_kind(&self, file_features: &FeaturesSupported) -> Option<SupportKind> {
        file_features
            .support_kind_if_not_enabled(FeatureKind::Lint)
            .and(file_features.support_kind_if_not_enabled(FeatureKind::Format))
            .and(file_features.support_kind_if_not_enabled(FeatureKind::Assist))
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        None
    }

    fn is_ci(&self) -> bool {
        true
    }

    fn as_diagnostic_category(&self) -> &'static Category {
        category!("ci")
    }

    fn should_skip_parse_errors(&self) -> bool {
        self.skip_parse_errors
    }

    fn requires_write_access(&self) -> bool {
        false
    }

    fn analyzer_selectors(&self) -> AnalyzerSelectors {
        AnalyzerSelectors {
            only: self.only.clone(),
            skip: self.skip.clone(),
        }
    }

    fn should_enforce_assist(&self) -> bool {
        self.enforce_assist
    }

    fn summary_phrase(&self, files: usize, duration: &Duration) -> MarkupBuf {
        SummaryVerbExecution.summary_verb("Checked", files, duration)
    }

    fn environment_to_reporter(&self) -> Option<CliReporter> {
        self.environment.map(|e| match e {
            ExecutionEnvironment::GitHub => CliReporter {
                kind: CliReporterKind::GitHub,
                destination: None,
            },
        })
    }
}

impl LoadEditorConfig for CiCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool {
        self.configuration
            .as_ref()
            .is_some_and(|c| c.use_editorconfig())
            || fs_configuration.use_editorconfig()
    }
}

impl TraversalCommand for CiCommandPayload {
    type ProcessFile = CheckProcessFile;

    fn command_name(&self) -> &'static str {
        "ci"
    }

    fn minimal_scan_kind(&self) -> Option<ScanKind> {
        None
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        _console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        let is_github = super::is_github_actions();

        Ok(Box::new(CiExecution {
            environment: if is_github {
                Some(ExecutionEnvironment::GitHub)
            } else {
                None
            },
            vcs_targeted: (false, self.changed).into(),
            enforce_assist: self.enforce_assist,
            skip_parse_errors: cli_options.skip_parse_errors,
            only: self.only.clone(),
            skip: self.skip.clone(),
        }))
    }

    fn merge_configuration(
        &mut self,
        loaded_configuration: Configuration,
        loaded_directory: Option<Utf8PathBuf>,
        _loaded_file: Option<Utf8PathBuf>,

        fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        let mut configuration =
            self.combine_configuration(loaded_directory, loaded_configuration, fs)?;

        let formatter = configuration
            .formatter
            .get_or_insert_with(FormatterConfiguration::default);

        if self.formatter_enabled.is_some() {
            formatter.enabled = self.formatter_enabled;
            formatter.format_with_errors = self.format_with_errors;
        }

        let linter = configuration
            .linter
            .get_or_insert_with(LinterConfiguration::default);

        if self.linter_enabled.is_some() {
            linter.enabled = self.linter_enabled;
        }

        let json = configuration
            .json
            .get_or_insert_with(JsonConfiguration::default);
        if self.json_parser.is_some() {
            json.parser.merge_with(self.json_parser.clone())
        }

        let css = configuration
            .css
            .get_or_insert_with(CssConfiguration::default);
        if self.css_parser.is_some() {
            css.parser.merge_with(self.css_parser.clone());
        }

        let assist = configuration
            .assist
            .get_or_insert_with(AssistConfiguration::default);

        if self.assist_enabled.is_some() {
            assist.enabled = self.assist_enabled;
        }

        if let Some(mut conf) = self.configuration.clone() {
            if let Some(linter) = conf.linter.as_mut() {
                // Don't overwrite rules from the CLI configuration.
                // Otherwise, rules that are disabled in the config file might
                // become re-enabled due to the defaults included in the CLI
                // configuration.
                linter.rules = None;
            }
            if let Some(assist) = conf.assist.as_mut() {
                // Don't overwrite actions from the CLI configuration.
                // Otherwise, actions that are disabled in the config file might
                // become re-enabled due to the defaults included in the CLI
                // configuration.
                assist.actions = None
            }
            configuration.merge_with(conf);
        }

        Ok(configuration)
    }

    fn get_files_to_process(
        &self,
        fs: &dyn FileSystem,
        configuration: &Configuration,
    ) -> Result<Vec<OsString>, CliDiagnostic> {
        if self.changed {
            get_changed_files(fs, configuration, self.since.as_deref())
        } else {
            Ok(self.paths.clone())
        }
    }

    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        if self.formatter_enabled.is_some_and(|v| !v.value())
            && self.linter_enabled.is_some_and(|v| !v.value())
            && self.assist_enabled.is_some_and(|v| !v.value())
        {
            return Err(CliDiagnostic::incompatible_end_configuration(
                "Formatter, linter and assist are disabled, can't perform the command. At least one feature needs to be enabled. This is probably and error.",
            ));
        }
        if self.since.is_some() && !self.changed {
            return Err(CliDiagnostic::incompatible_arguments(
                "--since",
                "--changed",
                "In order to use --since, you must also use --changed.",
            ));
        }
        Ok(())
    }
}
