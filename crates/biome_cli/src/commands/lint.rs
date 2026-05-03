use super::{FixFileModeOptions, determine_fix_file_mode};
use crate::CliDiagnostic;
use crate::cli_options::CliOptions;
use crate::commands::get_files_to_process_with_cli_options;
use crate::runner::execution::{AnalyzerSelectors, Execution, VcsTargeted};
use crate::runner::impls::commands::traversal::TraversalCommand;
use crate::runner::impls::executions::summary_verb::SummaryVerbExecution;
use crate::runner::impls::process_file::lint_and_assist::LintAssistProcessFile;
use biome_configuration::analyzer::AnalyzerSelector;
use biome_configuration::css::{CssLinterConfiguration, CssParserConfiguration};
use biome_configuration::graphql::GraphqlLinterConfiguration;
use biome_configuration::javascript::JsLinterConfiguration;
use biome_configuration::json::{JsonLinterConfiguration, JsonParserConfiguration};
use biome_configuration::vcs::VcsConfiguration;
use biome_configuration::{Configuration, FilesConfiguration, LinterConfiguration};
use biome_console::{Console, MarkupBuf};
use biome_deserialize::Merge;
use biome_diagnostics::{Category, category};
use biome_fs::FileSystem;
use biome_service::configuration::ProjectScanComputer;
use biome_service::workspace::{
    FeatureKind, FeatureName, FeaturesBuilder, FeaturesSupported, FixFileMode, ScanKind,
    SupportKind,
};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8PathBuf;
use std::ffi::OsString;
use std::time::Duration;

pub(crate) struct LintCommandPayload {
    pub(crate) write: bool,
    pub(crate) fix: bool,
    pub(crate) unsafe_: bool,
    pub(crate) suppress: bool,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) linter_configuration: Option<LinterConfiguration>,
    pub(crate) vcs_configuration: Option<VcsConfiguration>,
    pub(crate) files_configuration: Option<FilesConfiguration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) only: Vec<AnalyzerSelector>,
    pub(crate) skip: Vec<AnalyzerSelector>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) staged: bool,
    pub(crate) changed: bool,
    pub(crate) since: Option<String>,
    pub(crate) javascript_linter: Option<JsLinterConfiguration>,
    pub(crate) json_linter: Option<JsonLinterConfiguration>,
    pub(crate) css_linter: Option<CssLinterConfiguration>,
    pub(crate) graphql_linter: Option<GraphqlLinterConfiguration>,
    pub(crate) json_parser: Option<JsonParserConfiguration>,
    pub(crate) css_parser: Option<CssParserConfiguration>,
    pub(crate) profile_rules: bool,
}

struct LintExecution {
    /// The type of fixes that should be applied when analyzing a file.
    ///
    /// It's [None] if the `lint` command is called without `--apply` or `--apply-suggested`
    /// arguments.
    fix_file_mode: Option<FixFileMode>,
    /// An optional tuple.
    /// 1. The virtual path to the file
    /// 2. The content of the file
    stdin_file_path: Option<String>,
    /// Run only the given rule or group of rules.
    /// If the severity level of a rule is `off`,
    /// then the severity level of the rule is set to `error` if it is a recommended rule or `warn` otherwise.
    only: Vec<AnalyzerSelector>,
    /// Skip the given rule or group of rules by setting the severity level of the rules to `off`.
    /// This option takes precedence over `--only`.
    skip: Vec<AnalyzerSelector>,
    /// A flag to know vcs integrated options such as `--staged` or `--changed` are enabled
    vcs_targeted: VcsTargeted,
    /// Suppress existing diagnostics with a `// biome-ignore` comment
    suppress: bool,
    /// Explanation for suppressing diagnostics with `--suppress` and `--reason`
    suppression_reason: Option<String>,
    /// It skips parse errors
    skip_parse_errors: bool,
}

impl Execution for LintExecution {
    fn wanted_features(&self) -> FeatureName {
        FeaturesBuilder::new().with_linter().build()
    }

    fn not_requested_features(&self) -> FeatureName {
        FeaturesBuilder::new()
            .with_formatter()
            .with_assist()
            .with_search()
            .build()
    }

    fn can_handle(&self, features: FeaturesSupported) -> bool {
        features.supports_lint()
    }

    fn supports_kind(&self, file_features: &FeaturesSupported) -> Option<SupportKind> {
        Some(file_features.support_kind_for(FeatureKind::Lint))
    }

    fn is_vcs_targeted(&self) -> bool {
        self.vcs_targeted.changed || self.vcs_targeted.staged
    }

    fn get_stdin_file_path(&self) -> Option<&str> {
        self.stdin_file_path.as_deref()
    }

    fn scan_kind_computer(&self, computer: ProjectScanComputer) -> ScanKind {
        computer
            .with_rule_selectors(self.skip.as_ref(), self.only.as_ref())
            .compute()
    }

    fn as_diagnostic_category(&self) -> &'static Category {
        category!("lint")
    }

    fn as_fix_file_mode(&self) -> Option<FixFileMode> {
        self.fix_file_mode
    }

    fn should_skip_parse_errors(&self) -> bool {
        self.skip_parse_errors
    }

    fn suppress(&self) -> bool {
        self.suppress
    }

    fn suppression_reason(&self) -> Option<&str> {
        self.suppression_reason.as_deref()
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

    fn summary_phrase(&self, files: usize, duration: &Duration) -> MarkupBuf {
        SummaryVerbExecution.summary_verb("Checked", files, duration)
    }

    fn is_lint(&self) -> bool {
        true
    }
}

impl TraversalCommand for LintCommandPayload {
    type ProcessFile = LintAssistProcessFile;

    fn command_name(&self) -> &'static str {
        "lint"
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
        let fix_file_mode = determine_fix_file_mode(FixFileModeOptions {
            write: self.write,
            fix: self.fix,
            unsafe_: self.unsafe_,
            suppress: self.suppress,
            suppression_reason: self.suppression_reason.clone(),
        })?;

        if self.profile_rules {
            biome_analyze::profiling::enable();
        }

        Ok(Box::new(LintExecution {
            fix_file_mode,
            stdin_file_path: self.stdin_file_path.clone(),
            only: self.only.clone(),
            skip: self.skip.clone(),
            vcs_targeted: (self.staged, self.changed).into(),
            suppress: self.suppress,
            suppression_reason: self.suppression_reason.clone(),
            skip_parse_errors: cli_options.skip_parse_errors,
        }))
    }

    fn merge_configuration(
        &mut self,
        mut loaded_configuration: Configuration,
        _loaded_directory: Option<Utf8PathBuf>,
        _loaded_file: Option<Utf8PathBuf>,
        _fs: &dyn FileSystem,
        _console: &mut dyn Console,
    ) -> Result<Configuration, WorkspaceError> {
        loaded_configuration.merge_with(Configuration {
            linter: if loaded_configuration
                .linter
                .as_ref()
                .is_some_and(LinterConfiguration::is_enabled)
            {
                None
            } else {
                if let Some(linter) = self.linter_configuration.as_mut() {
                    // Don't overwrite rules from the CLI configuration.
                    linter.rules = None;
                }
                self.linter_configuration.clone()
            },
            files: self.files_configuration.clone(),
            vcs: self.vcs_configuration.clone(),
            ..Default::default()
        });

        let css = loaded_configuration
            .css
            .get_or_insert_with(Default::default);
        if self.css_linter.is_some() {
            css.linter.merge_with(self.css_linter.clone());
        }
        if self.css_parser.is_some() {
            css.parser.merge_with(self.css_parser.clone());
        }

        if self.graphql_linter.is_some() {
            let graphql = loaded_configuration
                .graphql
                .get_or_insert_with(Default::default);
            graphql.linter.merge_with(self.graphql_linter.clone());
        }
        if self.javascript_linter.is_some() {
            let javascript = loaded_configuration
                .javascript
                .get_or_insert_with(Default::default);
            javascript.linter.merge_with(self.javascript_linter.clone());
        }
        let json = loaded_configuration
            .json
            .get_or_insert_with(Default::default);
        if self.json_linter.is_some() {
            json.linter.merge_with(self.json_linter.clone());
        }
        if self.json_parser.is_some() {
            json.parser.merge_with(self.json_parser.clone());
        }

        Ok(loaded_configuration)
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
}
