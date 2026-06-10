use super::{FixFileModeOptions, determine_fix_file_mode, get_files_to_process_with_cli_options};
use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::runner::execution::{AnalyzerSelectors, Execution, VcsTargeted};
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
    FeatureKind, FeatureName, FeaturesBuilder, FeaturesSupported, FixFileMode, ScanKind,
    SupportKind,
};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;
use std::time::Duration;

pub(crate) struct CheckCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) formatter_enabled: Option<FormatterEnabled>,
    pub(crate) linter_enabled: Option<LinterEnabled>,
    pub(crate) assist_enabled: Option<AssistEnabled>,
    pub(crate) enforce_assist: bool,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
    pub(crate) format_with_errors: Option<FormatWithErrorsEnabled>,
    pub(crate) json_parser: Option<JsonParserConfiguration>,
    pub(crate) css_parser: Option<CssParserConfiguration>,
    pub(crate) only: Vec<AnalyzerSelector>,
    pub(crate) skip: Vec<AnalyzerSelector>,
    pub(crate) watch: bool,
    pub(crate) profile_rules: bool,
}

struct CheckExecution {
    /// The type of fixes that should be applied when analyzing a file.
    ///
    /// It's [None] if the `check` command is called without `--apply` or `--apply-suggested`
    /// arguments.
    fix_file_mode: Option<FixFileMode>,
    /// An optional tuple.
    /// 1. The virtual path to the file
    /// 2. The content of the file
    stdin_file_path: Option<String>,
    /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
    vcs_targeted: VcsTargeted,

    /// Whether assist diagnostics should be promoted to error, and fail the CLI
    enforce_assist: bool,

    /// It skips parse errors
    skip_parse_errors: bool,

    /// Run only the given lint rule, assist action, group of rules and actions, or domain
    only: Vec<AnalyzerSelector>,
    /// Skip the given lint rule, assist action, group of rules and actions, or domain
    skip: Vec<AnalyzerSelector>,
}

impl Execution for CheckExecution {
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
        self.stdin_file_path.as_deref()
    }

    fn is_check(&self) -> bool {
        true
    }

    fn as_diagnostic_category(&self) -> &'static Category {
        category!("check")
    }

    fn is_safe_fixes_enabled(&self) -> bool {
        self.fix_file_mode
            .is_some_and(|fix_mode| fix_mode == FixFileMode::SafeFixes)
    }

    fn is_safe_and_unsafe_fixes_enabled(&self) -> bool {
        self.fix_file_mode
            .is_some_and(|fix_mode| fix_mode == FixFileMode::SafeAndUnsafeFixes)
    }

    fn as_fix_file_mode(&self) -> Option<FixFileMode> {
        self.fix_file_mode
    }

    fn should_skip_parse_errors(&self) -> bool {
        self.skip_parse_errors
    }

    fn requires_write_access(&self) -> bool {
        self.fix_file_mode.is_some()
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
}

impl LoadEditorConfig for CheckCommandPayload {
    fn should_load_editor_config(&self, fs_configuration: &Configuration) -> bool {
        self.configuration
            .as_ref()
            .is_some_and(|c| c.use_editorconfig())
            || fs_configuration.use_editorconfig()
    }
}

impl TraversalCommand for CheckCommandPayload {
    type ProcessFile = CheckProcessFile;

    fn command_name(&self) -> &'static str {
        "check"
    }

    fn is_watch_mode(&self) -> bool {
        self.watch
    }

    fn minimal_scan_kind(&self) -> Option<ScanKind> {
        None
    }

    fn invocation_configuration(&self) -> Option<Configuration> {
        let formatter = (self.formatter_enabled.is_some() || self.format_with_errors.is_some())
            .then(|| FormatterConfiguration {
                enabled: self.formatter_enabled,
                format_with_errors: self.format_with_errors,
                ..Default::default()
            });
        let linter = self.linter_enabled.is_some().then(|| LinterConfiguration {
            enabled: self.linter_enabled,
            ..Default::default()
        });
        let assist = self.assist_enabled.is_some().then(|| AssistConfiguration {
            enabled: self.assist_enabled,
            ..Default::default()
        });

        if formatter.is_none()
            && linter.is_none()
            && assist.is_none()
            && self.json_parser.is_none()
            && self.css_parser.is_none()
        {
            None
        } else {
            Some(Configuration {
                formatter,
                linter,
                assist,
                json: self.json_parser.clone().map(|parser| JsonConfiguration {
                    parser: Some(parser),
                    ..Default::default()
                }),
                css: self.css_parser.clone().map(|parser| CssConfiguration {
                    parser: Some(parser),
                    ..Default::default()
                }),
                ..Default::default()
            })
        }
    }

    fn get_execution(
        &self,
        cli_options: &CliOptions,
        _console: &mut dyn Console,
        _workspace: &dyn Workspace,
    ) -> Result<Box<dyn Execution>, CliDiagnostic> {
        let fix_file_mode = determine_fix_file_mode(FixFileModeOptions {
            write: self.write,
            suppress: false,
            suppression_reason: None,
            fix: self.fix,
            unsafe_: self.unsafe_,
        })?;

        if self.profile_rules {
            biome_analyze::profiling::enable();
        }

        Ok(Box::new(CheckExecution {
            fix_file_mode,
            stdin_file_path: self.stdin_file_path.clone(),
            vcs_targeted: (self.staged, self.changed).into(),
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
        let paths = get_files_to_process_with_cli_options(
            self.since.as_deref(),
            self.changed,
            self.staged,
            fs,
            configuration,
        )?
        .unwrap_or(self.paths.clone());

        Ok(paths)
    }

    fn check_incompatible_arguments(&self) -> Result<(), CliDiagnostic> {
        if self.is_watch_mode() {
            if self.fix {
                return Err(CliDiagnostic::incompatible_arguments(
                    "--watch",
                    "--fix",
                    "Applying code fixes is not available in watch mode.",
                ));
            }

            if self.write {
                return Err(CliDiagnostic::incompatible_arguments(
                    "--watch",
                    "--write",
                    "Applying code fixes is not available in watch mode.",
                ));
            }
        }

        Ok(())
    }
}
