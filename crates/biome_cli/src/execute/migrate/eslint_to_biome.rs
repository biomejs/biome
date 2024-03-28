use biome_deserialize::StringSet;
use biome_js_analyze::lint::style::no_restricted_globals;
use biome_service::configuration::{self as biome};

use super::{eslint, eslint_any_rule_to_biome::migrate_eslint_any_rule, eslint_typescript};

/// This modules includes implementations for converting an ESlint config to a Biome config.
///
/// The conversion relies on:
/// - the generated [super::eslint_any_rule_to_biome::migrate_eslint_any_rule]
///   module that relies on Biome's rule metadata to determine
///   the equivalent Biome's rule of an Eslint rule
/// - hand-written handling of Biome rules that have options in the current module.

#[derive(Debug, Clone)]
pub(crate) struct MigrationOptions {
    /// Migrate inspired rules from eslint and its plugins?
    pub(crate) include_inspired: bool,
    /// Migrate nursery rules from eslint and its plugins?
    pub(crate) include_nursery: bool,
}

#[derive(Debug, Default)]
pub(crate) struct MigrationResults {
    // Rules that were successfuly migrated
    pub(crate) migrated_rules: Vec<&'static str>,
    // Rules that have no equivalent in Biome
    pub(crate) unsupported_rules: Vec<String>,
    // Inspired rules that were not migrated because `include_inspired` is disabled
    pub(crate) inspired_rules: Vec<&'static str>,
    // Nursery rules that were not migrated because `include_nursery` is disabled
    pub(crate) nursery_rules: Vec<&'static str>,
}

impl eslint::ConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome::PartialConfiguration, MigrationResults) {
        let mut results = MigrationResults::default();
        let mut biome_config = biome::PartialConfiguration::default();
        if !self.globals.is_empty() {
            let globals = self
                .globals
                .into_iter()
                .filter_map(|(global_name, global_conf)| {
                    global_conf.is_enabled().then_some(global_name)
                })
                .collect::<StringSet>();
            let js_config = biome::PartialJavascriptConfiguration {
                globals: Some(globals),
                ..Default::default()
            };
            biome_config.javascript = Some(js_config)
        }
        let mut linter = biome::PartialLinterConfiguration::default();
        if !self.ignore_patterns.is_empty() {
            let ignore = self.ignore_patterns.into_iter().collect::<StringSet>();
            linter.ignore = Some(ignore);
        }
        if !self.rules.is_empty() {
            linter.rules = Some(self.rules.into_biome_rules(options, &mut results));
        }
        if !self.overrides.is_empty() {
            let mut overrides = biome::Overrides::default();
            for override_elt in self.overrides {
                let mut override_pattern = biome::OverridePattern::default();
                if !override_elt.globals.is_empty() {
                    let globals = override_elt.globals.into_keys().collect::<StringSet>();
                    let js_config = biome::PartialJavascriptConfiguration {
                        globals: Some(globals),
                        ..Default::default()
                    };
                    override_pattern.javascript = Some(js_config)
                }
                if !override_elt.excluded_files.is_empty() {
                    override_pattern.ignore =
                        Some(override_elt.excluded_files.into_iter().collect());
                }
                if !override_elt.files.is_empty() {
                    override_pattern.ignore = Some(override_elt.files.into_iter().collect());
                }
                if !override_elt.rules.is_empty() {
                    override_pattern.linter = Some(biome::overrides::OverrideLinterConfiguration {
                        rules: Some(override_elt.rules.into_biome_rules(options, &mut results)),
                        ..Default::default()
                    });
                }
                overrides.0.push(override_pattern);
            }
            biome_config.overrides = Some(overrides);
        }
        biome_config.linter = Some(linter);
        (biome_config, results)
    }
}

impl eslint::Rules {
    pub(crate) fn into_biome_rules(
        self,
        options: &MigrationOptions,
        results: &mut MigrationResults,
    ) -> biome::Rules {
        let mut rules = biome::Rules::default();
        for eslint_rule in self {
            migrate_eslint_rule(&mut rules, eslint_rule, options, results);
        }
        rules
    }
}

/// Look for an equivalent Biome rule for ESlint `rule`,
/// and then mutate `rules` if a equivalent rule is found.
/// Also, takes care of Biome's rules with options.
fn migrate_eslint_rule(
    rules: &mut biome_service::Rules,
    rule: eslint::Rule,
    opts: &MigrationOptions,
    results: &mut MigrationResults,
) {
    let name = rule.name();
    match rule {
        eslint::Rule::Any(name, severity) => {
            let _ = migrate_eslint_any_rule(rules, &name, severity, opts, results);
        }
        eslint::Rule::NoRestrictedGlobals(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let severity = conf.severity();
                let globals = conf.into_vec().into_iter().map(|g| g.into_name());
                let group = rules.style.get_or_insert_with(Default::default);
                group.no_restricted_globals = Some(biome_service::RuleConfiguration::WithOptions(
                    biome_service::RuleWithOptions {
                        level: severity.into(),
                        options: Box::new(no_restricted_globals::RestrictedGlobalsOptions {
                            denied_globals: globals.collect(),
                        }),
                    },
                ));
            }
        }
        eslint::Rule::Jsxa11yArioaRoles(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.a11y.get_or_insert_with(Default::default);
                    group.use_valid_aria_role =
                        Some(biome_service::RuleConfiguration::WithOptions(
                            biome_service::RuleWithOptions {
                                level: severity.into(),
                                options: Box::new((*rule_options).into()),
                            },
                        ));
                }
            }
        }
        eslint::Rule::TypeScriptArrayType(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.style.get_or_insert_with(Default::default);
                    group.use_consistent_array_type =
                        Some(biome_service::RuleConfiguration::WithOptions(
                            biome_service::RuleWithOptions {
                                level: severity.into(),
                                options: rule_options.into(),
                            },
                        ));
                }
            }
        }
        eslint::Rule::TypeScriptNamingConvention(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let severity = conf.severity();
                let options = eslint_typescript::NamingConventionOptions::override_default(
                    conf.into_vec().into_iter().map(|v| *v),
                );
                let group = rules.style.get_or_insert_with(Default::default);
                group.use_naming_convention = Some(biome_service::RuleConfiguration::WithOptions(
                    biome_service::RuleWithOptions {
                        level: severity.into(),
                        options: options.into(),
                    },
                ));
            }
        }
        eslint::Rule::UnicornFilenameCase(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let group = rules.style.get_or_insert_with(Default::default);
                group.use_filenaming_convention = Some(
                    biome_service::RuleConfiguration::WithOptions(biome_service::RuleWithOptions {
                        level: conf.severity().into(),
                        options: Box::new(conf.option_or_default().into()),
                    }),
                );
            }
        }
    }
}
