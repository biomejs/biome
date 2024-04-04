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

#[derive(Debug, Clone)]
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
        let mut global_ignores = StringSet::default();
        let mut global_config_object = eslint_eslint::FlatConfigObject::default();
        // First determine the base configuration
        for flat_config_object in self.0 {
            if flat_config_object.is_global_ignores() {
                global_ignores.extend(flat_config_object.ignores);
            } else if flat_config_object.is_global_config() {
                global_config_object.merge_with(flat_config_object);
            } else {
                let mut override_pattern = biome_config::OverridePattern::default();
                if let Some(language_options) = flat_config_object.language_options {
                    let globals = language_options.globals.enabled().collect::<StringSet>();
                    let js_config = biome_config::PartialJavascriptConfiguration {
                        globals: Some(globals),
                        ..Default::default()
                    };
                    override_pattern.javascript = Some(js_config)
                }
                if !flat_config_object.ignores.is_empty() {
                    override_pattern.ignore =
                        Some(flat_config_object.ignores.into_iter().collect());
                }
                if !flat_config_object.files.is_empty() {
                    override_pattern.include = Some(flat_config_object.files.into_iter().collect());
                }
                if let Some(rules) = flat_config_object.rules {
                    if !rules.is_empty() {
                        override_pattern.linter = Some(biome_config::OverrideLinterConfiguration {
                            rules: Some(rules.into_biome_rules(options, &mut results)),
                            ..Default::default()
                        });
                    }
                }
                overrides.0.push(override_pattern);
            }
        }
        if !overrides.0.is_empty() {
            biome_config.overrides = Some(overrides);
        }
        debug_assert!(global_config_object.is_global_config());
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
        if !global_ignores.is_empty() {
            linter.ignore = Some(global_ignores);
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
            let ignore = self.ignore_patterns.into_iter().collect::<StringSet>();
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
        for eslint_rule in self {
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
        eslint_eslint::Rule::NoRestrictedGlobals(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let severity = conf.severity();
                let globals = conf.into_vec().into_iter().map(|g| g.into_name());
                let group = rules.style.get_or_insert_with(Default::default);
                group.no_restricted_globals = Some(biome_config::RuleConfiguration::WithOptions(
                    biome_config::RuleWithOptions {
                        level: severity.into(),
                        options: Box::new(no_restricted_globals::RestrictedGlobalsOptions {
                            denied_globals: globals.collect(),
                        }),
                    },
                ));
            }
        }
        eslint_eslint::Rule::Jsxa11yArioaRoles(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                if let eslint_eslint::RuleConf::Option(severity, rule_options) = conf {
                    let group = rules.a11y.get_or_insert_with(Default::default);
                    group.use_valid_aria_role = Some(biome_config::RuleConfiguration::WithOptions(
                        biome_config::RuleWithOptions {
                            level: severity.into(),
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
                let options = eslint_typescript::NamingConventionOptions::override_default(
                    conf.into_vec().into_iter().map(|v| *v),
                );
                let group = rules.style.get_or_insert_with(Default::default);
                group.use_naming_convention = Some(biome_config::RuleConfiguration::WithOptions(
                    biome_config::RuleWithOptions {
                        level: severity.into(),
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
