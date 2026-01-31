use crate::cli_options::CliOptions;
use biome_configuration::analyzer::AnalyzerSelector;
use biome_console::MarkupBuf;
use biome_diagnostics::Category;
use biome_fs::BiomePath;
use biome_grit_patterns::GritTargetLanguage;
use biome_service::configuration::ProjectScanComputer;
use biome_service::workspace::{
    FeatureName, FeaturesSupported, FixFileMode, PatternId, ScanKind, SupportKind,
};
use biome_service::{Workspace, WorkspaceError};
use camino::{Utf8Path, Utf8PathBuf};
use std::time::Duration;
use tracing::info;

pub(crate) trait Execution: Send + Sync + std::panic::RefUnwindSafe {
    /// The features that this command requires to be enabled.
    fn wanted_features(&self) -> FeatureName;

    /// The features that this command should be disabled.
    fn not_requested_features(&self) -> FeatureName;

    /// Whether this command can handle the incoming file given its features.
    fn can_handle(&self, features: FeaturesSupported) -> bool;

    /// Hook used after the crawling, and before processing the final output.
    fn on_post_crawl(&self, _workspace: &dyn Workspace) -> Result<(), WorkspaceError> {
        Ok(())
    }

    fn get_max_diagnostics(&self, cli_options: &CliOptions) -> u32 {
        if cli_options
            .cli_reporter
            .iter()
            .any(|reporter| !reporter.is_default())
        {
            info!(
                "Removing the limit of --max-diagnostics, because of a reporter list contains a reporter different from the default one."
            );
            u32::MAX
        } else {
            cli_options.max_diagnostics.into()
        }
    }

    /// Whether this command should be aware of the VCS integration
    fn is_vcs_targeted(&self) -> bool;

    /// Used by [crate::runner::ProcessFile::execute] to determine which kind of support kind the file has
    fn supports_kind(&self, file_features: &FeaturesSupported) -> Option<SupportKind>;

    /// It should returns the value of `--stdin-file-path`
    fn get_stdin_file_path(&self) -> Option<&str>;

    /// Derives the [ScanKind] for this execution
    fn scan_kind_computer(&self, computer: ProjectScanComputer) -> ScanKind {
        computer.compute()
    }

    fn compute_scan_kind(
        &self,
        target_known_paths: &[String],
        working_dir: &Utf8Path,
        scan_kind: ScanKind,
    ) -> ScanKind {
        match scan_kind {
            ScanKind::KnownFiles => {
                let target_paths = target_known_paths
                    .iter()
                    .map(|path| BiomePath::new(working_dir.join(path)))
                    .collect();
                ScanKind::TargetedKnownFiles {
                    target_paths,
                    descend_from_targets: true,
                }
            }
            _ => scan_kind,
        }
    }

    /// If the execution is running in CI mode, it will return true.
    /// Otherwise, it will return false.
    ///
    /// At the moment, CI is equal to `biome ci`
    fn is_ci(&self) -> bool {
        false
    }

    /// `biome check` command
    fn is_check(&self) -> bool {
        false
    }

    /// `biome lint` command
    fn is_lint(&self) -> bool {
        false
    }

    /// The [Category] that should be used when running the command.
    fn as_diagnostic_category(&self) -> &'static Category;

    /// Whether the execution should apply safe fixes
    fn is_safe_fixes_enabled(&self) -> bool {
        false
    }

    /// Whether the execution should apply safe and unsafe fixes
    fn is_safe_and_unsafe_fixes_enabled(&self) -> bool {
        false
    }

    /// `biome search` command
    fn is_search(&self) -> bool {
        false
    }

    /// The kind of fix mode to apply
    fn as_fix_file_mode(&self) -> Option<FixFileMode> {
        None
    }

    /// The value of `--skip-parse-errors`
    fn should_skip_parse_errors(&self) -> bool {
        false
    }

    /// The value of `--suppress`
    fn suppress(&self) -> bool {
        false
    }

    /// The value of `--suppression-reason`
    fn suppression_reason(&self) -> Option<&str> {
        None
    }

    /// Whether this command needs to write on the file system
    fn requires_write_access(&self) -> bool;

    /// Values of `--only` and `--skip`
    fn analyzer_selectors(&self) -> AnalyzerSelectors;

    /// Whether assists should be enforced
    fn should_enforce_assist(&self) -> bool {
        false
    }

    /// `biome format` command
    fn is_format(&self) -> bool {
        false
    }

    /// The search target language
    fn search_language(&self) -> Option<GritTargetLanguage> {
        None
    }
    /// The search pattern to search for.
    fn search_pattern(&self) -> Option<&PatternId> {
        None
    }

    /// Used when printing summary
    fn summary_phrase(&self, files: usize, duration: &Duration) -> MarkupBuf;
}

#[derive(Debug, Default, Clone)]
pub(crate) struct AnalyzerSelectors {
    pub(crate) only: Vec<AnalyzerSelector>,
    pub(crate) skip: Vec<AnalyzerSelector>,
}

/// A type that holds the information to execute the CLI via `stdin
#[derive(Debug, Clone)]
pub struct Stdin(
    /// The virtual path to the file
    Utf8PathBuf,
    /// The content of the file
    String,
);

impl Stdin {
    pub(crate) fn as_path(&self) -> &Utf8Path {
        self.0.as_path()
    }

    pub(crate) fn as_content(&self) -> &str {
        self.1.as_str()
    }
}

impl From<(Utf8PathBuf, String)> for Stdin {
    fn from((path, content): (Utf8PathBuf, String)) -> Self {
        Self(path, content)
    }
}

#[derive(Default, Debug, Clone)]
pub struct VcsTargeted {
    pub staged: bool,
    pub changed: bool,
}

impl From<(bool, bool)> for VcsTargeted {
    fn from((staged, changed): (bool, bool)) -> Self {
        Self { staged, changed }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionEnvironment {
    GitHub,
}
