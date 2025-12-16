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
    fn to_feature(&self) -> FeatureName;

    fn can_handle(&self, features: FeaturesSupported) -> bool;

    fn on_post_crawl(&self, _workspace: &dyn Workspace) -> Result<(), WorkspaceError> {
        Ok(())
    }

    fn get_max_diagnostics(&self, cli_options: &CliOptions) -> u32 {
        if cli_options.reporter.is_default() {
            cli_options.max_diagnostics.into()
        } else {
            info!(
                "Removing the limit of --max-diagnostics, because of a reporter different from the default one: {}",
                cli_options.reporter
            );
            u32::MAX
        }
    }

    /// Whether this command should be aware of the VCS integration
    fn is_vcs_targeted(&self) -> bool;

    fn supports_kind(&self, file_features: &FeaturesSupported) -> Option<SupportKind>;

    // TODO implement for commands that support it
    fn get_stdin_file_path(&self) -> Option<&str>;

    // TODO: implement this for the linter which contains only and skip

    /// Derives the [ScanKind] for this execution
    fn scan_kind_computer(&self, computer: ProjectScanComputer) -> ScanKind {
        computer.compute()
    }

    // TODO: implement this for migrate
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
    // TODO: implement for `biome ci`
    fn is_ci(&self) -> bool {
        false
    }

    /// Whether the command is running in check mode e.g., no `--write`
    // TODO: implement for `check`, `format` and `lint`
    fn is_check(&self) -> bool {
        false
    }

    fn is_lint(&self) -> bool {
        false
    }

    fn as_diagnostic_category(&self) -> &'static Category;

    // TODO: implement this for check and lint
    fn is_safe_fixes_enabled(&self) -> bool {
        false
    }

    // TODO: implement this for check and lint
    fn is_safe_and_unsafe_fixes_enabled(&self) -> bool {
        false
    }

    // TODO: implement this for search command
    fn is_search(&self) -> bool {
        false
    }

    // TODO: implement this for all commands
    // || category.name().starts_with("lint/")
    // || category.name().starts_with("suppressions/")
    //                         || category.name().starts_with("assist/")
    //                         || category.name().starts_with("plugin")
    // category.name() == "parse"
    // category.name() == "format"
    fn should_report(&self, category: &Category) -> bool;

    // TODO: implement for check and lint
    fn as_fix_file_mode(&self) -> Option<FixFileMode> {
        None
    }

    fn should_skip_parse_errors(&self) -> bool {
        false
    }

    // TODO: implement for check and lint

    fn suppress(&self) -> bool {
        false
    }
    // TODO: implement for check and lint

    fn suppression_reason(&self) -> Option<&str> {
        None
    }

    // TODO implement this for all commands
    fn requires_write_access(&self) -> bool;

    // TODO implement for lint and check commands
    fn analyzer_selectors(&self) -> AnalyzerSelectors;

    // TODO implement for the check command
    fn should_enforce_assist(&self) -> bool {
        false
    }

    fn is_format(&self) -> bool {
        false
    }

    fn search_language(&self) -> Option<GritTargetLanguage> {
        None
    }
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
