use crate::{Execution, TraversalMode};
use biome_analyze::{Queryable, RegistryVisitor, Rule, RuleDomain, RuleFilter, RuleGroup};
use biome_configuration::Configuration;
use biome_configuration::analyzer::{RuleDomainValue, RuleDomains, RuleSelector};
use biome_css_syntax::CssLanguage;
use biome_graphql_syntax::GraphqlLanguage;
use biome_js_syntax::JsLanguage;
use biome_json_syntax::JsonLanguage;
use biome_rowan::Language;
use biome_service::workspace::ScanKind;
use rustc_hash::FxHashSet;

/// It analyzes the CLI and the configuration to understand what kind of scanning Biome needs to execute.
///
/// Rules:
/// - CLI via `stdin` return [ScanKind::None]
/// - `biome format` return [ScanKind::KnownFiles] if VCS is enabled, otherwise [ScanKind::None]
/// - `biome lint`, `biome check` and `biome ci` may vary. It depends on whether the user has enabled rules that require the `RuleDomain::Project`.
///   If not, returns [ScanKind::KnownFiles] if VCS is enabled, otherwise [ScanKind::None]
pub(crate) fn compute_scan_kind(execution: &Execution, configuration: &Configuration) -> ScanKind {
    if execution.is_stdin() || execution.is_migrate() {
        return ScanKind::None;
    };

    if execution.is_format() {
        // There's no need to scan further known files if the VCS isn't enabled
        return if !configuration.use_ignore_file() {
            ScanKind::None
        } else {
            ScanKind::KnownFiles
        };
    };

    let lint_rules = configuration.get_linter_rules().as_enabled_rules();
    let domains = configuration.get_linter_domains();
    let mut requires_project_scan = RequiresProjectScan::new(&lint_rules, domains);

    if let TraversalMode::Lint { only, skip, .. } = execution.traversal_mode() {
        requires_project_scan = requires_project_scan
            .with_skip(skip.clone())
            .with_only(only.clone());
    }

    let result = requires_project_scan.compute();

    if result == ScanKind::KnownFiles {
        // There's no need to scan further known files if the VCS isn't enabled
        if !configuration.use_ignore_file() {
            ScanKind::None
        } else {
            result
        }
    } else {
        result
    }
}

struct RequiresProjectScan<'a> {
    requires_project_scan: bool,
    enabled_rules: &'a FxHashSet<RuleFilter<'a>>,
    domains: Option<&'a RuleDomains>,
    skip: Vec<RuleSelector>,
    only: Vec<RuleSelector>,
}

impl<'a> RequiresProjectScan<'a> {
    fn new(enabled_rules: &'a FxHashSet<RuleFilter<'a>>, domains: Option<&'a RuleDomains>) -> Self {
        Self {
            enabled_rules,
            requires_project_scan: false,
            domains,
            skip: vec![],
            only: vec![],
        }
    }

    fn with_only(mut self, only: Vec<RuleSelector>) -> Self {
        self.only = only;
        self
    }
    fn with_skip(mut self, skip: Vec<RuleSelector>) -> Self {
        self.skip = skip;
        self
    }

    fn compute(mut self) -> ScanKind {
        if let Some(domains) = &self.domains {
            for (domain, value) in domains.iter() {
                if domain == &RuleDomain::Project && value != &RuleDomainValue::None {
                    self.requires_project_scan = true;
                    break;
                }
            }
        }

        biome_graphql_analyze::visit_registry(&mut self);
        biome_css_analyze::visit_registry(&mut self);
        biome_json_analyze::visit_registry(&mut self);
        biome_js_analyze::visit_registry(&mut self);

        if self.requires_project_scan {
            ScanKind::Project
        } else {
            ScanKind::KnownFiles
        }
    }

    fn check_rule<R, L>(&mut self)
    where
        L: Language,
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let filter = RuleFilter::Rule(<R::Group as RuleGroup>::NAME, R::METADATA.name);
        let selector = RuleSelector::Rule(<R::Group as RuleGroup>::NAME, R::METADATA.name);
        if !self.only.is_empty() {
            if self.only.contains(&selector) {
                let domains = R::METADATA.domains;
                self.requires_project_scan |= domains.contains(&RuleDomain::Project);
            }
        } else if !self.skip.contains(&selector) && self.enabled_rules.contains(&filter) {
            let domains = R::METADATA.domains;
            self.requires_project_scan |= domains.contains(&RuleDomain::Project);
        }
    }
}

impl RegistryVisitor<JsLanguage> for RequiresProjectScan<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.check_rule::<R, JsLanguage>();
    }
}

impl RegistryVisitor<JsonLanguage> for RequiresProjectScan<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, JsonLanguage>();
    }
}

impl RegistryVisitor<CssLanguage> for RequiresProjectScan<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, CssLanguage>();
    }
}

impl RegistryVisitor<GraphqlLanguage> for RequiresProjectScan<'_> {
    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.check_rule::<R, GraphqlLanguage>();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TraversalMode, VcsTargeted};
    use biome_configuration::analyzer::{
        Correctness, RuleDomainValue, RuleDomains, SeverityOrGroup,
    };
    use biome_configuration::vcs::{VcsClientKind, VcsConfiguration};
    use biome_configuration::{
        LinterConfiguration, RuleConfiguration, RulePlainConfiguration, Rules,
    };
    use biome_service::projects::ProjectKey;
    use rustc_hash::FxHashMap;

    fn execution() -> Execution {
        Execution::new(TraversalMode::Check {
            project_key: ProjectKey::new(),
            enforce_assist: false,
            fix_file_mode: None,
            stdin: None,
            vcs_targeted: VcsTargeted::default(),
        })
    }

    #[test]
    fn should_scan_known_files_when_no_rules_are_enabled() {
        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                enabled: Some(false.into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let execution = execution();

        assert_eq!(
            compute_scan_kind(&execution, &configuration),
            ScanKind::None
        );
    }

    #[test]
    fn should_scan_project_project_domain_is_enabled() {
        let mut domains = FxHashMap::default();
        domains.insert(RuleDomain::Project, RuleDomainValue::Recommended);

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                domains: Some(RuleDomains(domains)),
                ..Default::default()
            }),
            ..Default::default()
        };

        let execution = execution();

        assert_eq!(
            compute_scan_kind(&execution, &configuration),
            ScanKind::Project
        );
    }

    #[test]
    fn should_scan_project_project_rule_is_enabled() {
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

        let execution = execution();

        assert_eq!(
            compute_scan_kind(&execution, &configuration),
            ScanKind::Project
        );
    }

    #[test]
    fn should_skip_project_rule_is_skipped() {
        let execution = Execution::new(TraversalMode::Lint {
            project_key: ProjectKey::new(),
            fix_file_mode: None,
            stdin: None,
            only: vec![],
            skip: vec![RuleSelector::Rule("correctness", "noPrivateImports")],

            vcs_targeted: VcsTargeted::default(),
            suppress: false,
            suppression_reason: None,
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
            ScanKind::None
        );
    }

    #[test]
    fn should_return_known_files_if_vcs_is_enabled() {
        let execution = Execution::new(TraversalMode::Lint {
            project_key: ProjectKey::new(),
            fix_file_mode: None,
            stdin: None,
            only: vec![],
            skip: vec![RuleSelector::Rule("correctness", "noPrivateImports")],

            vcs_targeted: VcsTargeted::default(),
            suppress: false,
            suppression_reason: None,
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
            ScanKind::KnownFiles
        );
    }

    #[test]
    fn should_return_project_if_project_rule_is_only() {
        let execution = Execution::new(TraversalMode::Lint {
            project_key: ProjectKey::new(),
            fix_file_mode: None,
            stdin: None,
            skip: vec![],
            only: vec![RuleSelector::Rule("correctness", "noPrivateImports")],
            vcs_targeted: VcsTargeted::default(),
            suppress: false,
            suppression_reason: None,
        });

        let configuration = Configuration {
            linter: Some(LinterConfiguration {
                rules: Some(Rules {
                    correctness: Some(SeverityOrGroup::Group(Correctness {
                        no_private_imports: Some(RuleConfiguration::Plain(
                            RulePlainConfiguration::Off,
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
            ScanKind::Project
        );
    }
}
