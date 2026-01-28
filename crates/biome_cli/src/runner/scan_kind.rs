use crate::runner::execution::Stdin;
use biome_configuration::Configuration;
use biome_fs::BiomePath;
use biome_service::workspace::ScanKind;
use camino::Utf8Path;

/// Returns a forced scan kind based on the given `execution`.
fn get_forced_scan_kind(
    stdin: Option<&Stdin>,
    root_configuration_dir: &Utf8Path,
    working_dir: &Utf8Path,
    maybe_scan_kind: Option<ScanKind>,
) -> Option<ScanKind> {
    if let Some(stdin) = stdin {
        let path = stdin.as_path();
        if path
            .parent()
            .is_some_and(|dir| dir == root_configuration_dir)
        {
            return Some(ScanKind::NoScanner);
        } else {
            return Some(ScanKind::TargetedKnownFiles {
                target_paths: vec![BiomePath::new(working_dir.join(path))],
                descend_from_targets: false,
            });
        }
    }

    maybe_scan_kind
}

/// Figures out the best (as in, most efficient) scan kind for the given execution.
///
/// Rules:
/// - When processing from `stdin`, we return [ScanKind::NoScanner] if the stdin
///   file path is in the directory of the root configuration, and
///   [ScanKind::TargetedKnownFiles] otherwise.
/// - Returns [ScanKind::KnownFiles] for `biome format`, `biome migrate`, and
///   `biome search`, because we know there is no use for project analysis with
///   these commands.
/// - If the linter is disabled, we don't ever return [ScanKind::Project], because
///   we don't need to scan the project in that case.
/// - Otherwise, we return the requested scan kind.
pub(crate) fn derive_best_scan_kind(
    requested_scan_kind: ScanKind,
    stdin: Option<&Stdin>,
    root_configuration_dir: &Utf8Path,
    working_dir: &Utf8Path,
    configuration: &Configuration,
    command_scan_kind: Option<ScanKind>,
) -> ScanKind {
    get_forced_scan_kind(
        stdin,
        root_configuration_dir,
        working_dir,
        command_scan_kind,
    )
    .unwrap_or({
        let required_minimum_scan_kind =
            if configuration.is_root() || configuration.use_ignore_file() {
                ScanKind::KnownFiles
            } else {
                ScanKind::NoScanner
            };
        if requested_scan_kind == ScanKind::NoScanner
            || (requested_scan_kind == ScanKind::Project && !configuration.is_linter_enabled())
        {
            // If we're here, it means we're executing `check`, `lint` or `ci`
            // and the linter is disabled or no projects rules have been enabled.
            // We scan known files if the configuration is a root or if the VCS integration is enabled
            required_minimum_scan_kind
        } else {
            requested_scan_kind
        }
    })
}
