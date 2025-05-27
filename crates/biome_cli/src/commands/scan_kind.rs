use crate::Execution;
use biome_configuration::Configuration;
use biome_service::workspace::ScanKind;

/// It analyzes the CLI and the configuration to understand what kind of scanning Biome needs to execute.
///
/// Rules:
/// - CLI via `stdin` return [ScanKind::None]
/// - `biome migrate` return [ScanKind:KnownFiles], so it can migrate all nested configuration files
/// - `biome format` return [ScanKind::KnownFiles] if VCS is enabled, otherwise [ScanKind::None]
/// -  `None` otherwise
pub(crate) fn compute_scan_kind(
    execution: &Execution,
    configuration: &Configuration,
) -> Option<ScanKind> {
    if execution.is_stdin() {
        Some(ScanKind::None)
    } else if execution.is_migrate() || configuration.is_root() {
        Some(ScanKind::KnownFiles)
    } else if execution.is_format() {
        // There's no need to scan further known files if the VCS isn't enabled
        if !configuration.use_ignore_file() {
            Some(ScanKind::None)
        } else {
            Some(ScanKind::KnownFiles)
        }
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TraversalMode, VcsTargeted};
    use biome_configuration::analyzer::{Correctness, RuleSelector, SeverityOrGroup};
    use biome_configuration::vcs::{VcsClientKind, VcsConfiguration};
    use biome_configuration::{
        LinterConfiguration, RuleConfiguration, RulePlainConfiguration, Rules,
    };
    use biome_service::workspace::ScanKind::KnownFiles;

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

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    correctness: Some(SeverityOrGroup::Group(Correctness {
                        no_private_imports: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Error,
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            compute_scan_kind(&execution, &configuration),
            Some(KnownFiles)
        );
    }

    #[test]
    fn should_return_known_files_if_vcs_is_enabled_when_formatting() {
        let execution = Execution::new(TraversalMode::Format {
            stdin: None,
            vcs_targeted: VcsTargeted::default(),
            skip_parse_errors: false,
            write: false,
        });

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    correctness: Some(SeverityOrGroup::Group(Correctness {
                        no_private_imports: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Error,
                        )),
                        ..Default::default()
                    })),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            vcs: Some(VcsConfiguration {
                enabled: Some(true.into()),
                client_kind: Some(VcsClientKind::Git),
                use_ignore_file: Some(true.into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        assert_eq!(
            compute_scan_kind(&execution, &configuration),
            Some(ScanKind::KnownFiles)
        );
    }
}
