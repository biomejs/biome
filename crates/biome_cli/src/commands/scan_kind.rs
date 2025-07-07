use crate::{Execution, TraversalMode};
use biome_fs::BiomePath;
use biome_service::workspace::ScanKind;
use camino::Utf8Path;

/// Returns a forced scan kind based on the given `execution`.
///
/// Rules:
/// - When processing from `stdin`, we return [ScanKind::NoScanner] if the stdin
///   file path is in the directory of the root configuration, and
///   [ScanKind::TargetedKnownFiles] otherwise.
/// - Returns [ScanKind::KnownFiles] for `biome format`, `biome migrate`, and
///   `biome search`, because we know there is no use for project analysis with
///   these commands.
/// - Returns `None` otherwise.
pub(crate) fn get_forced_scan_kind(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TraversalMode, VcsTargeted};
    use biome_configuration::analyzer::RuleSelector;

    #[test]
    fn should_return_none_for_lint_command() {
        let execution = Execution::new(TraversalMode::Lint {
            fix_file_mode: None,
            stdin: None,
            only: vec![],
            skip: vec![RuleSelector::Rule("correctness", "noPrivateImports")],

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
}
