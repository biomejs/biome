use crate::TraversalMode;
use crate::cli_options::CliOptions;
use crate::execute::Stdin;
use biome_configuration::Configuration;
use biome_diagnostics::Category;
use biome_fs::BiomePath;
use biome_service::configuration::ProjectScanComputer;
use biome_service::workspace::{FeatureName, FeaturesSupported, ScanKind};
use biome_service::{Workspace, WorkspaceError};
use camino::Utf8Path;
use tracing::info;

pub trait Execution: Send + Sync {
    const DIAGNOSTICS_CATEGORY: &'static Category;

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

    // TODO implement for commands that support it
    fn get_stdin_file_path(&self) -> Option<&str> {
        None
    }

    fn as_stdin_file(&self) -> Option<&Stdin> {
        None
    }

    // TODO: implement this for the linter which contains only and skip
    fn scan_kind_computer<'a>(&self, configuration: &'a Configuration) -> ProjectScanComputer<'a> {
        ProjectScanComputer::new(configuration)
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
        true
    }

    fn as_diagnostic_category(&self) -> Category {
        *Self::DIAGNOSTICS_CATEGORY.clone()
    }
}
