use crate::{Execution, TraversalMode};
use biome_configuration::Configuration;
use biome_fs::BiomePath;
use biome_service::workspace::ScanKind;
use camino::Utf8Path;

/// Returns a forced scan kind based on the given `execution`.
fn get_forced_scan_kind(
    execution: &Execution,
    root_configuration_dir: &Utf8Path,
    working_dir: &Utf8Path,
) -> Option<ScanKind> {
    if let Some(stdin) = execution.as_stdin_file() {
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

    // We want to keep the `match`, so if we add new traversal modes,
    // the compiler will error, and we will need to handle the new variant
    match execution.traversal_mode() {
        TraversalMode::Format { .. }
        | TraversalMode::Migrate { .. }
        | TraversalMode::Search { .. } => Some(ScanKind::KnownFiles),
        // These traversals might enable lint rules that require project rules,
        // so we need to return `None` so we can use the `ScanKind` returned by the workspace
        TraversalMode::Lint { .. } | TraversalMode::Check { .. } | TraversalMode::CI { .. } => None,
    }
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
    execution: &Execution,
    root_configuration_dir: &Utf8Path,
    working_dir: &Utf8Path,
    configuration: &Configuration,
) -> ScanKind {
    get_forced_scan_kind(execution, root_configuration_dir, working_dir).unwrap_or({
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TraversalMode, VcsTargeted};
    use biome_configuration::LinterConfiguration;
    use biome_configuration::analyzer::RuleSelector;

    #[test]
    fn should_return_none_for_lint_command() {
        let execution = Execution::new(TraversalMode::Lint {
            fix_file_mode: None,
            stdin: None,
            only: vec![],
            skip: vec![RuleSelector::Rule("correctness", "noPrivateImports").into()],

            vcs_targeted: VcsTargeted::default(),
            suppress: false,
            suppression_reason: None,
            skip_parse_errors: false,
        });

        let root_dir = Utf8Path::new("/");
        assert_eq!(get_forced_scan_kind(&execution, root_dir, root_dir), None);
    }

    #[test]
    fn should_return_known_files_for_format_command() {
        let execution = Execution::new(TraversalMode::Format {
            skip_parse_errors: false,
            write: true,
            stdin: None,
            vcs_targeted: VcsTargeted::default(),
        });

        let root_dir = Utf8Path::new("/");
        assert_eq!(
            get_forced_scan_kind(&execution, root_dir, root_dir),
            Some(ScanKind::KnownFiles)
        );
    }

    #[test]
    fn should_not_scan_project_if_linter_disabled() {
        let execution = Execution::new(TraversalMode::Check {
            fix_file_mode: None,
            stdin: None,

            vcs_targeted: VcsTargeted::default(),
            skip_parse_errors: false,
            enforce_assist: true,
        });

        let config = Configuration {
            linter: Some(LinterConfiguration {
                enabled: Some(false.into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let root_dir = Utf8Path::new("/");
        assert_ne!(
            derive_best_scan_kind(ScanKind::Project, &execution, root_dir, root_dir, &config),
            ScanKind::Project
        );
    }
}
