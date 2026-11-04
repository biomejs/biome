use biome_configuration::{Configuration, ConfigurationPathHint};
use biome_deserialize::Merge;
use biome_diagnostics::Severity;
use biome_fs::BiomePath;
use biome_resolver::FsWithResolverProxy;
use biome_service::configuration::{LoadedConfiguration, read_config};
use biome_service::workspace::ScanKind;
use camino::Utf8Path;

/// Returns a forced scan kind based on the given `execution`.
fn get_forced_scan_kind(
    stdin_file_path: Option<&Utf8Path>,
    root_configuration_dir: &Utf8Path,
    working_dir: &Utf8Path,
    maybe_scan_kind: Option<ScanKind>,
) -> Option<ScanKind> {
    if let Some(stdin_file_path) = stdin_file_path {
        if stdin_file_path
            .parent()
            .is_some_and(|dir| dir == root_configuration_dir)
        {
            return Some(ScanKind::NoScanner);
        } else {
            return Some(ScanKind::TargetedKnownFiles {
                target_paths: vec![BiomePath::new(working_dir.join(stdin_file_path))],
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
    stdin: Option<&Utf8Path>,
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

/// Returns the strongest scan kind that the nested configurations at
/// `configuration_paths` require, if it indexes more files than `scan_kind`.
///
/// Loads them as the workspace's `update_project_config_files` does: skips the
/// configuration in `project_dir`, `"root": true` configurations and
/// configurations with errors (the scan reports those), and merges a
/// configuration that extends the root into `root_configuration`.
pub(crate) fn derive_nested_scan_kind(
    fs: &dyn FsWithResolverProxy,
    project_dir: &Utf8Path,
    root_configuration: &Configuration,
    configuration_paths: &[BiomePath],
    scan_kind: &ScanKind,
    compute: impl Fn(&Configuration) -> ScanKind,
) -> Option<ScanKind> {
    let mut nested_scan_kind = None;
    let nested_paths = configuration_paths
        .iter()
        .filter(|path| path.parent() != Some(project_dir));
    for path in nested_paths {
        let Ok(Some(payload)) = read_config(
            fs,
            ConfigurationPathHint::FromWorkspace(path.to_path_buf()),
            false,
        ) else {
            continue;
        };
        let Ok(loaded_configuration) = LoadedConfiguration::try_from_payload(Some(payload), fs)
        else {
            continue;
        };
        if loaded_configuration
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
        {
            continue;
        }
        let configuration = loaded_configuration.resolved_configuration();
        if configuration.is_root() {
            continue;
        }
        let configuration = if configuration.extends_root() {
            let mut merged_configuration = root_configuration.clone();
            merged_configuration.merge_with(configuration);
            merged_configuration
        } else {
            configuration
        };

        let candidate = compute(&configuration);
        if project_scan_level(&candidate)
            > nested_scan_kind
                .as_ref()
                .map_or(project_scan_level(scan_kind), project_scan_level)
        {
            let is_type_aware = candidate.is_type_aware();
            nested_scan_kind = Some(candidate);
            if is_type_aware {
                break;
            }
        }
    }
    nested_scan_kind
}

/// Ranks the scan kinds that index the whole project; the others rank `0`.
fn project_scan_level(scan_kind: &ScanKind) -> u8 {
    match scan_kind {
        ScanKind::TypeAware => 2,
        ScanKind::Project => 1,
        _ => 0,
    }
}
