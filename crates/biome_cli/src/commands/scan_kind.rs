use crate::Execution;
use biome_service::workspace::ScanKind;

/// Returns a forced scan kind based on the given `execution`.
///
/// Rules:
/// - Returns [ScanKind::None] when processing from `stdin`. When using `stdin`,
///   we don't know the input's real path, so we can't match nested configs or
///   resolve import paths, meaning there's no use for the scanner.
/// - Returns [ScanKind::KnownFiles] for `biome format`, `biome migrate`, and
///   `biome search` because we know there is no use for project analysis with
///   these commands.
/// - Returns `None` otherwise.
pub(crate) fn get_forced_scan_kind(execution: &Execution) -> Option<ScanKind> {
    if execution.is_stdin() {
        Some(ScanKind::None)
    } else if execution.is_migrate() || execution.is_format() || execution.is_search() {
        Some(ScanKind::KnownFiles)
    } else {
        None
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

        assert_eq!(get_forced_scan_kind(&execution), None);
    }

    #[test]
    fn should_return_known_files_for_format_command() {
        let execution = Execution::new(TraversalMode::Format {
            skip_parse_errors: false,
            write: true,
            stdin: None,

            vcs_targeted: VcsTargeted::default(),
        });

        assert_eq!(get_forced_scan_kind(&execution), Some(ScanKind::KnownFiles));
    }
}
