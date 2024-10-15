use biome_configuration::{self as biome_config};
use biome_deserialize::{Merge, StringSet};
use biome_js_analyze::lint::style::no_restricted_globals;

use super::{eslint_any_rule_to_biome::migrate_eslint_any_rule, eslint_eslint, eslint_typescript};

/// This modules includes implementations for converting an ESLint config to a Biome config.
///
/// The conversion relies on:
/// - the generated [super::eslint_any_rule_to_biome::migrate_eslint_any_rule]
///   module that relies on Biome's rule metadata to determine
///   the equivalent Biome's rule of an Eslint rule
/// - hand-written handling of Biome rules that have options in the current module.

#[derive(Clone, Debug, Default)]
pub(crate) struct MigrationOptions {
    /// Migrate inspired rules from eslint and its plugins?
    pub(crate) include_inspired: bool,
    /// Migrate nursery rules from eslint and its plugins?
    pub(crate) include_nursery: bool,
}

#[derive(Debug, Default)]
pub(crate) struct MigrationResults {
    // Contains inspired rules that were not migrated because `include_inspired` is disabled
    pub(crate) has_inspired_rules: bool,
}

impl eslint_eslint::AnyConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome_config::PartialConfiguration, MigrationResults) {
        match self {
            Self::Flat(config) => config.into_biome_config(options),
            Self::Legacy(config) => config.into_biome_config(options),
        }
    }
}

impl eslint_eslint::FlatConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome_config::PartialConfiguration, MigrationResults) {
        let mut results = MigrationResults::default();
        let mut biome_config = biome_config::PartialConfiguration::default();
        let mut linter = biome_config::PartialLinterConfiguration::default();
        let mut overrides = biome_config::Overrides::default();
        let global_config_object = if self.0.len() == 1 {
            // If there is a single config object, then we use it as the global config
            self.0.into_iter().next().unwrap()
        } else {
            let mut global_config_object = eslint_eslint::FlatConfigObject::default();
            for flat_config_object in self.0 {
                if flat_config_object.is_global_ignores() {
                    global_config_object
                        .ignores
                        .extend(flat_config_object.ignores);
                } else if flat_config_object.is_global_config() {
                    global_config_object.merge_with(flat_config_object);
                } else {
                    let mut override_pat = biome_config::OverridePattern::default();
                    if let Some(language_options) = flat_config_object.language_options {
                        let globals = language_options.globals.enabled().collect::<StringSet>();
                        let js_config = biome_config::PartialJavascriptConfiguration {
                            globals: Some(globals),
                            ..Default::default()
                        };
                        override_pat.javascript = Some(js_config)
                    }
                    if !flat_config_object.ignores.is_empty() {
                        override_pat.ignore =
                            Some(flat_config_object.ignores.into_iter().collect());
                    }
                    if !flat_config_object.files.is_empty() {
                        override_pat.include = Some(flat_config_object.files.into_iter().collect());
                    }
                    if let Some(rules) = flat_config_object.rules {
                        if !rules.is_empty() {
                            override_pat.linter = Some(biome_config::OverrideLinterConfiguration {
                                rules: Some(rules.into_biome_rules(options, &mut results)),
                                ..Default::default()
                            });
                        }
                    }
                    overrides.0.push(override_pat);
                }
            }
            if !overrides.0.is_empty() {
                biome_config.overrides = Some(overrides);
            }
            global_config_object
        };
        let mut rules = if let Some(rules) = global_config_object.rules {
            rules.into_biome_rules(options, &mut results)
        } else {
            biome_config::Rules::default()
        };
        if let Some(language_options) = global_config_object.language_options {
            let globals = language_options.globals.enabled().collect::<StringSet>();
            let js_config = biome_config::PartialJavascriptConfiguration {
                globals: Some(globals),
                ..Default::default()
            };
            biome_config.javascript = Some(js_config)
        }
        rules.recommended = Some(false);
        linter.rules = Some(rules);
        if !global_config_object.ignores.is_empty() {
            linter.ignore = Some(global_config_object.ignores.into_iter().collect());
        }
        if !global_config_object.files.is_empty() {
            linter.include = Some(global_config_object.files.into_iter().collect());
        }
        biome_config.linter = Some(linter);
        (biome_config, results)
    }
}

impl eslint_eslint::LegacyConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome_config::PartialConfiguration, MigrationResults) {
        let mut results = MigrationResults::default();
        let mut biome_config = biome_config::PartialConfiguration::default();
        if !self.globals.is_empty() {
            let globals = self.globals.enabled().collect::<StringSet>();
            let js_config = biome_config::PartialJavascriptConfiguration {
                globals: Some(globals),
                ..Default::default()
            };
            biome_config.javascript = Some(js_config)
        }
        let mut linter = biome_config::PartialLinterConfiguration::default();
        let mut rules = self.rules.into_biome_rules(options, &mut results);
        rules.recommended = Some(false);
        linter.rules = Some(rules);
        if !self.ignore_patterns.is_empty() {
            let ignore = self
                .ignore_patterns
                .into_iter()
                .map(|p| p.0)
                .collect::<StringSet>();
            linter.ignore = Some(ignore);
        }
        if !self.overrides.is_empty() {
            let mut overrides = biome_config::Overrides::default();
            for override_elt in self.overrides {
                let mut override_pattern = biome_config::OverridePattern::default();
                if !override_elt.globals.is_empty() {
                    let globals = override_elt.globals.enabled().collect::<StringSet>();
                    let js_config = biome_config::PartialJavascriptConfiguration {
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
                    override_pattern.include = Some(override_elt.files.into_iter().collect());
                }
                if !override_elt.rules.is_empty() {
                    override_pattern.linter = Some(biome_config::OverrideLinterConfiguration {
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

impl eslint_eslint::Rules {
    pub(crate) fn into_biome_rules(
        self,
        options: &MigrationOptions,
        results: &mut MigrationResults,
    ) -> biome_config::Rules {
        let mut rules = biome_config::Rules::default();
        for eslint_rule in self.0 {
            migrate_eslint_rule(&mut rules, eslint_rule, options, results);
        }
        rules
    }
}

/// Look for an equivalent Biome rule for ESLint `rule`,
/// and then mutate `rules` if a equivalent rule is found.
/// Also, takes care of Biome's rules with options.
fn migrate_eslint_rule(
    rules: &mut biome_config::Rules,
    rule: eslint_eslint::Rule,
    opts: &MigrationOptions,
    results: &mut MigrationResults,
) {
    let name = rule.name();
    match rule {
        eslint_eslint::Rule::Any(name, severity) => {
            let _ = migrate_eslint_any_rule(rules, &name, severity, opts, results);
        }
        eslint_eslint::Rule::NoConsole(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint_eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.suspicious.get_or_insert_with(Default::default);
                    group.no_console = Some(biome_config::RuleFixConfiguration::WithOptions(
                        biome_config::RuleWithFixOptions {
                            level: severity.into(),
                            fix: None,
                            options: Box::new((*rule_options).into()),
                        },
                    ));
                }
            }
        }
        eslint_eslint::Rule::NoRestrictedGlobals(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let severity = conf.severity();
                let globals = conf
                    .into_vec()
                    .into_iter()
                    .map(|g| g.into_name().into_boxed_str());
                let group = rules.style.get_or_insert_with(Default::default);
                group.no_restricted_globals = Some(biome_config::RuleConfiguration::WithOptions(
                    biome_config::RuleWithOptions {
                        level: severity.into(),
                        options: Box::new(no_restricted_globals::RestrictedGlobalsOptions {
                            denied_globals: globals.collect::<Vec<_>>().into_boxed_slice(),
                        }),
                    },
                ));
            }
        }
        eslint_eslint::Rule::Jsxa11yArioaRoles(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint_eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.a11y.get_or_insert_with(Default::default);
                    group.use_valid_aria_role =
                        Some(biome_config::RuleFixConfiguration::WithOptions(
                            biome_config::RuleWithFixOptions {
                                level: severity.into(),
                                fix: None,
                                options: Box::new((*rule_options).into()),
                            },
                        ));
                }
            }
        }
        eslint_eslint::Rule::TypeScriptArrayType(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint_eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.style.get_or_insert_with(Default::default);
                    group.use_consistent_array_type =
                        Some(biome_config::RuleFixConfiguration::WithOptions(
                            biome_config::RuleWithFixOptions {
                                level: severity.into(),
                                fix: None,
                                options: rule_options.into(),
                            },
                        ));
                }
            }
        }
        eslint_eslint::Rule::TypeScriptExplicitMemberAccessibility(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint_eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.nursery.get_or_insert_with(Default::default);
                    group.use_consistent_member_accessibility =
                        Some(biome_config::RuleConfiguration::WithOptions(
                            biome_config::RuleWithOptions {
                                level: severity.into(),
                                options: rule_options.into(),
                            },
                        ));
                }
            }
        }
        eslint_eslint::Rule::TypeScriptNamingConvention(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let severity = conf.severity();
                let options = eslint_typescript::NamingConventionOptions::new(
                    conf.into_vec().into_iter().map(|v| *v),
                );
                let group = rules.style.get_or_insert_with(Default::default);
                group.use_naming_convention =
                    Some(biome_config::RuleFixConfiguration::WithOptions(
                        biome_config::RuleWithFixOptions {
                            level: severity.into(),
                            fix: None,
                            options: options.into(),
                        },
                    ));
            }
        }
        eslint_eslint::Rule::UnicornFilenameCase(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let group = rules.style.get_or_insert_with(Default::default);
                group.use_filenaming_convention = Some(
                    biome_config::RuleConfiguration::WithOptions(biome_config::RuleWithOptions {
                        level: conf.severity().into(),
                        options: Box::new(conf.option_or_default().into()),
                    }),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eslint_eslint::*;
    use std::borrow::Cow;

    #[test]
    fn flat_config_single_config_object() {
        let flat_config = FlatConfigData(vec![FlatConfigObject {
            files: vec!["*.js".to_string()],
            ignores: vec!["*.test.js".to_string()],
            language_options: None,
            rules: Some(Rules(
                [Rule::Any(Cow::Borrowed("eqeqeq"), Severity::Error)]
                    .into_iter()
                    .collect(),
            )),
        }]);
        let (biome_config, _) = flat_config.into_biome_config(&MigrationOptions::default());

        assert!(biome_config.files.is_none());
        assert!(biome_config.overrides.is_none());
        assert!(biome_config.formatter.is_none());
        assert!(biome_config.organize_imports.is_none());
        let linter = biome_config.linter.unwrap();
        assert_eq!(
            linter.include,
            Some(["*.js".to_string()].into_iter().collect())
        );
        assert_eq!(
            linter.ignore,
            Some(["*.test.js".to_string()].into_iter().collect())
        );
        assert!(linter.rules.is_some());
    }

    #[test]
    fn flat_config_multiple_config_object() {
        let flat_config = FlatConfigData(vec![
            FlatConfigObject {
                files: vec![],
                ignores: vec!["*.test.js".to_string()],
                language_options: None,
                rules: None,
            },
            FlatConfigObject {
                files: vec![],
                ignores: vec![],
                language_options: None,
                rules: Some(Rules(
                    [Rule::Any(Cow::Borrowed("eqeqeq"), Severity::Error)]
                        .into_iter()
                        .collect(),
                )),
            },
            FlatConfigObject {
                files: vec![],
                ignores: vec!["*.spec.js".to_string()],
                language_options: None,
                rules: None,
            },
            FlatConfigObject {
                files: vec!["*.ts".to_string()],
                ignores: vec![],
                language_options: None,
                rules: Some(Rules(
                    [Rule::Any(Cow::Borrowed("eqeqeq"), Severity::Off)]
                        .into_iter()
                        .collect(),
                )),
            },
        ]);
        let (biome_config, _) = flat_config.into_biome_config(&MigrationOptions::default());

        assert!(biome_config.files.is_none());
        assert!(biome_config.formatter.is_none());
        assert!(biome_config.organize_imports.is_none());
        let linter = biome_config.linter.unwrap();
        assert!(linter.include.is_none());
        assert_eq!(
            linter.ignore,
            Some(
                ["*.test.js".to_string(), "*.spec.js".to_string()]
                    .into_iter()
                    .collect()
            )
        );
        assert_eq!(
            linter.rules.unwrap().suspicious.unwrap().no_double_equals,
            Some(biome_config::RuleFixConfiguration::Plain(
                biome_config::RulePlainConfiguration::Error
            ))
        );
        let overrides = biome_config.overrides.unwrap();
        assert_eq!(overrides.0.len(), 1);
        let override0 = overrides.0.into_iter().next().unwrap();
        assert_eq!(
            override0.include,
            Some(["*.ts".to_string()].into_iter().collect())
        );
        assert!(override0.ignore.is_none());
        assert_eq!(
            override0
                .linter
                .unwrap()
                .rules
                .unwrap()
                .suspicious
                .unwrap()
                .no_double_equals,
            Some(biome_config::RuleFixConfiguration::Plain(
                biome_config::RulePlainConfiguration::Off
            ))
        );
    }
}
