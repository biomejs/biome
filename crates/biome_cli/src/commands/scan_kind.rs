use crate::Execution;
use biome_analyze::{Queryable, RegistryVisitor, Rule, RuleDomain, RuleFilter, RuleGroup};
use biome_configuration::Configuration;
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
/// - `biome format` return [ScanKind::KnownFiles]
/// - `biome lint`, `biome check` and `biome ci` varies. It depends on whether the user has enabled rules that require the `RuleDomain::Project`
pub(crate) fn compute_scan_kind(execution: &Execution, configuration: &Configuration) -> ScanKind {
    if execution.is_stdin() || execution.is_migrate() {
        return ScanKind::None;
    };

    if execution.is_format() {
        return ScanKind::KnownFiles;
    };

    let lint_rules = configuration.get_linter_rules().as_enabled_rules();
    let requires_project_scan = RequiresProjectScan::new(&lint_rules);

    requires_project_scan.compute()
}

struct RequiresProjectScan<'a> {
    requires_project_scan: bool,
    enabled_rules: &'a FxHashSet<RuleFilter<'a>>,
}

impl<'a> RequiresProjectScan<'a> {
    fn new(enabled_rules: &'a FxHashSet<RuleFilter<'a>>) -> Self {
        Self {
            enabled_rules,
            requires_project_scan: false,
        }
    }

    fn compute(mut self) -> ScanKind {
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
        if let Some(_) = self.enabled_rules.get(&filter) {
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
    use biome_configuration::{
        LinterConfiguration, RuleConfiguration, RulePlainConfiguration, Rules,
    };
    use biome_service::projects::ProjectKey;
    use rustc_hash::FxHashMap;

    fn execution() -> Execution {
        Execution::new(TraversalMode::Check {
            stdin: None,
            project_key: ProjectKey::new(),
            enforce_assist: false,
            fix_file_mode: None,
            vcs_targeted: VcsTargeted {
                changed: false,
                staged: false,
            },
        })
    }

    #[test]
    fn should_scan_known_files_when_no_rules_are_enabled() {
        let mut domains = FxHashMap::default();
        domains.insert(RuleDomain::Project, RuleDomainValue::Recommended);

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
            ScanKind::Project
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
}
