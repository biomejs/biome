use std::collections::BTreeSet;

use super::{eslint_any_rule_to_biome::migrate_eslint_any_rule, eslint_eslint, eslint_typescript};
use biome_configuration::analyzer::SeverityOrGroup;
use biome_configuration::{self as biome_config};
use biome_console::markup;
use biome_deserialize::Merge;
use biome_diagnostics::Location;
use biome_rule_options::no_restricted_globals;
use rustc_hash::FxHashMap;

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

/// Sorted ESlint stylistic rules.
/// The array is sorted to allow binary search.
const ESLINT_STYLISTIC_RULES: &[&str] = &[
    "array-bracket-newline",
    "array-bracket-spacing",
    "array-element-newline",
    "arrow-body-style",
    "arrow-parens",
    "arrow-spacing",
    "block-spacing",
    "brace-style",
    "capitalized-comments",
    "comma-dangle",
    "comma-spacing",
    "comma-style",
    "computed-property-spacing",
    "dot-location",
    "eol-last",
    "func-call-spacing",
    "function-call-argument-newline",
    "function-paren-newline",
    "generator-star-spacing",
    "implicit-arrow-linebreak",
    "indent",
    "indent-legacy",
    "jsx-quotes",
    "key-spacing",
    "keyword-spacing",
    "line-comment-position",
    "linebreak-style",
    "lines-around-comment",
    "lines-around-directive",
    "lines-between-class-members",
    "max-len",
    "max-statements-per-line",
    "multiline-comment-style",
    "multiline-ternary",
    "new-parens",
    "newline-after-var",
    "newline-before-return",
    "newline-per-chained-call",
    "no-confusing-arrow",
    "no-extra-parens",
    "no-extra-semi",
    "no-floating-decimal",
    "no-mixed-operators",
    "no-mixed-spaces-and-tabs",
    "no-multiple-empty-lines",
    "no-spaced-func",
    "no-tabs",
    "no-trailing-spaces",
    "no-whitespace-before-property",
    "nonblock-statement-body-position",
    "object-curly-newline",
    "object-curly-spacing",
    "object-property-newline",
    "one-var-declaration-per-line",
    "operator-linebreak",
    "padded-blocks",
    "padding-line-between-statements",
    "quote-props",
    "quotes",
    "rest-spread-spacing",
    "semi",
    "semi-spacing",
    "semi-style",
    "space-before-blocks",
    "space-before-function-paren",
    "space-in-parens",
    "space-infix-ops",
    "space-unary-ops",
    "spaced-comment",
    "switch-colon-spacing",
    "template-curly-spacing",
    "template-tag-spacing",
    "wrap-iife",
    "wrap-regex",
    "yield-star-spacing",
];

#[derive(Debug, Default)]
pub(crate) struct MigrationResults {
    /// Path to the migrated ESlint configuration
    pub(crate) eslint_path: Option<Box<str>>,
    /// Is the Biome configuration updated?
    pub(crate) write: bool,
    // Contains inspired rules that were not migrated because `include_inspired` is disabled
    pub(crate) inspired: BTreeSet<EslintRuleName>,
    pub(crate) nursery: BTreeSet<EslintRuleName>,
    pub(crate) migrated: BTreeSet<EslintRuleName>,
    /// Stylistic rules that are not supported on purpose.
    pub(crate) stylistic: BTreeSet<EslintRuleName>,
    pub(crate) unsupported: BTreeSet<EslintRuleName>,
}
impl MigrationResults {
    pub(crate) fn add(&mut self, sourced_rule: &str, status: RuleMigrationResult) {
        let sourced = EslintRuleName::from_str(sourced_rule);
        match status {
            RuleMigrationResult::Migrated => {
                self.migrated.insert(sourced);
            }
            RuleMigrationResult::Inspired => {
                self.inspired.insert(sourced);
            }
            RuleMigrationResult::Nursery => {
                self.nursery.insert(sourced);
            }
            RuleMigrationResult::Unsupported => {
                if sourced.rule_name.starts_with("@stylistic/")
                    || (sourced.plugin_name.is_none()
                        && ESLINT_STYLISTIC_RULES
                            .binary_search(&sourced.rule_name.as_ref())
                            .is_ok())
                {
                    self.stylistic.insert(sourced);
                } else {
                    self.unsupported.insert(sourced);
                };
            }
        }
    }

    pub(crate) fn rule_count(&self) -> usize {
        self.migrated.len()
            + self.inspired.len()
            + self.nursery.len()
            + self.stylistic.len()
            + self.unsupported.len()
    }
}
impl biome_diagnostics::Diagnostic for MigrationResults {
    fn category(&self) -> Option<&'static biome_diagnostics::Category> {
        Some(biome_diagnostics::category!("migrate"))
    }

    fn severity(&self) -> biome_diagnostics::Severity {
        biome_diagnostics::Severity::Information
    }

    fn location(&self) -> biome_diagnostics::Location<'_> {
        let mut builder = Location::builder();
        if let Some(path) = self.eslint_path.as_ref() {
            builder = builder.resource(path);
        }
        builder.build()
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        let count = self.rule_count();
        if count != 0 {
            let migrated_count = self.migrated.len()
                + if self.write {
                    0
                } else {
                    self.inspired.len() + self.nursery.len()
                };
            let migrated_percent = migrated_count * 100 / count;
            let verb = if self.write { "have been" } else { "can be" };
            fmt.write_markup(markup! { {migrated_percent}"% ("{migrated_count}"/"{count}") of the rules "{verb}" migrated." })
        } else {
            fmt.write_markup(markup! { "No rules to migrate." })
        }
    }

    fn advices(&self, visitor: &mut dyn biome_diagnostics::Visit) -> std::io::Result<()> {
        if !self.migrated.is_empty() && self.migrated.len() != self.rule_count() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &if self.write {
                    markup! { "Migrated rules:" }
                } else {
                    markup! { "Rules that can be migrated:" }
                },
            )?;
            let list: Vec<_> = self
                .migrated
                .iter()
                .map(|item| item as &dyn biome_console::fmt::Display)
                .collect();
            visitor.record_list(list.as_slice())?;
        }
        if !self.inspired.is_empty() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &markup! { "Rules that can be migrated to an inspired rule using "<Emphasis>"--include-inspired"</Emphasis>":" },
            )?;
            let list: Vec<_> = self
                .inspired
                .iter()
                .map(|item| item as &dyn biome_console::fmt::Display)
                .collect();
            visitor.record_list(list.as_slice())?;
        }
        if !self.inspired.is_empty() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &markup! { "Rules that can be migrated to a nursery rule using "<Emphasis>"--include-nursery"</Emphasis>":" },
            )?;
            let list: Vec<_> = self
                .nursery
                .iter()
                .map(|item| item as &dyn biome_console::fmt::Display)
                .collect();
            visitor.record_list(list.as_slice())?;
        }
        if !self.stylistic.is_empty() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &markup! { "Stylistic rules that the formatter may support (manual migration required):" },
            )?;
            let list: Vec<_> = self
                .stylistic
                .iter()
                .map(|item| item as &dyn biome_console::fmt::Display)
                .collect();
            visitor.record_list(list.as_slice())?;
        }
        if !self.unsupported.is_empty() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &markup! { "Unsupported rules:" },
            )?;
            let list: Vec<_> = self
                .unsupported
                .iter()
                .map(|item| item as &dyn biome_console::fmt::Display)
                .collect();
            visitor.record_list(list.as_slice())?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum RuleMigrationResult {
    /// A rule that has been migrated.
    Migrated,
    /// A rule that could be migrated if `--include-inspired` was passed
    Inspired,
    /// A rule that could be migrated if `--include-nursery` was passed
    Nursery,
    /// An unsupported rule
    Unsupported,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct EslintRuleName {
    plugin_name: Option<Box<str>>,
    rule_name: Box<str>,
}
impl EslintRuleName {
    fn from_str(s: &str) -> Self {
        if let Some((plugin_name, rule_name)) = s.split_once('/') {
            Self {
                plugin_name: Some(plugin_name.into()),
                rule_name: rule_name.into(),
            }
        } else {
            Self {
                plugin_name: None,
                rule_name: s.into(),
            }
        }
    }
}
impl std::fmt::Display for EslintRuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rule_name = &self.rule_name;
        if let Some(plugin_name) = &self.plugin_name {
            f.write_fmt(format_args!("{plugin_name}/{rule_name}"))
        } else {
            f.write_str(rule_name)
        }
    }
}
impl biome_console::fmt::Display for EslintRuleName {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter) -> std::io::Result<()> {
        fmt.write_fmt(format_args!("{self}"))
    }
}

impl eslint_eslint::AnyConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome_config::Configuration, MigrationResults) {
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
    ) -> (biome_config::Configuration, MigrationResults) {
        let mut results = MigrationResults::default();
        let mut biome_config = biome_config::Configuration::default();
        let mut linter = biome_config::LinterConfiguration::default();
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
                        let globals = language_options.globals.enabled().collect();
                        let js_config = biome_config::JsConfiguration {
                            globals: Some(globals),
                            ..Default::default()
                        };
                        override_pat.javascript = Some(js_config)
                    }
                    let includes =
                        to_biome_includes(&flat_config_object.files, &flat_config_object.ignores);
                    override_pat.includes = (!includes.is_empty())
                        .then_some(biome_configuration::OverrideGlobs::Globs(includes.into()));
                    if let Some(rules) = flat_config_object.rules
                        && !rules.is_empty()
                    {
                        override_pat.linter = Some(biome_config::OverrideLinterConfiguration {
                            rules: Some(rules.into_biome_rules(options, &mut results)),
                            ..Default::default()
                        });
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
            let globals = language_options
                .globals
                .enabled()
                .collect::<rustc_hash::FxHashSet<_>>();
            let js_config = biome_config::JsConfiguration {
                globals: Some(globals),
                ..Default::default()
            };
            biome_config.javascript = Some(js_config)
        }
        rules.recommended = Some(false);
        linter.rules = Some(rules);
        let includes =
            to_biome_includes(&global_config_object.files, &global_config_object.ignores);
        linter.includes = (!includes.is_empty()).then_some(includes);
        biome_config.linter = Some(linter);
        (biome_config, results)
    }
}

impl eslint_eslint::LegacyConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome_config::Configuration, MigrationResults) {
        let mut results = MigrationResults::default();
        let mut biome_config = biome_config::Configuration::default();
        if !self.globals.is_empty() {
            let globals = self.globals.enabled().collect::<rustc_hash::FxHashSet<_>>();
            let js_config = biome_config::JsConfiguration {
                globals: Some(globals),
                ..Default::default()
            };
            biome_config.javascript = Some(js_config)
        }
        let mut linter = biome_config::LinterConfiguration::default();
        let mut rules = self.rules.into_biome_rules(options, &mut results);
        rules.recommended = Some(false);
        linter.rules = Some(rules);
        let includes = to_biome_includes(&[] as &[&str], self.ignore_patterns.as_slice());
        linter.includes = (!includes.is_empty()).then_some(includes);
        if !self.overrides.is_empty() {
            let mut overrides = biome_config::Overrides::default();
            for override_elt in self.overrides {
                let mut override_pattern = biome_config::OverridePattern::default();
                if !override_elt.globals.is_empty() {
                    let globals = override_elt
                        .globals
                        .enabled()
                        .collect::<rustc_hash::FxHashSet<_>>();
                    let js_config = biome_config::JsConfiguration {
                        globals: Some(globals),
                        ..Default::default()
                    };
                    override_pattern.javascript = Some(js_config)
                }
                let includes = to_biome_includes(&override_elt.files, &override_elt.excluded_files);
                override_pattern.includes = (!includes.is_empty())
                    .then_some(biome_configuration::OverrideGlobs::Globs(includes.into()));
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
/// and then mutate `rules` if an equivalent rule is found.
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
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results)
                && let eslint_eslint::RuleConf::Option(severity, rule_options) = conf
            {
                let group = rules.suspicious.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
                    group.no_console = Some(biome_config::RuleFixConfiguration::WithOptions(
                        biome_config::RuleWithFixOptions {
                            level: severity.into(),
                            fix: None,
                            options: *Box::new((*rule_options).into()),
                        },
                    ));
                }
            }
        }
        eslint_eslint::Rule::NoRestrictedGlobals(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let severity = conf.severity();
                let globals = conf.into_vec().into_iter().map(|g| {
                    (
                        g.name().to_string().into_boxed_str(),
                        g.message()
                            .map_or_else(
                                || "TODO: Add a custom message here.".to_string(),
                                |m| m.to_string(),
                            )
                            .into_boxed_str(),
                    )
                });
                let group = rules.style.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
                    group.no_restricted_globals =
                        Some(biome_config::RuleConfiguration::WithOptions(
                            biome_config::RuleWithOptions {
                                level: severity.into(),
                                options: *Box::new(
                                    no_restricted_globals::NoRestrictedGlobalsOptions {
                                        denied_globals: globals.collect::<FxHashMap<_, _>>(),
                                    },
                                ),
                            },
                        ));
                }
            }
        }
        eslint_eslint::Rule::Jsxa11yArioaRoles(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results)
                && let eslint_eslint::RuleConf::Option(severity, rule_options) = conf
            {
                let group = rules.a11y.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
                    group.use_valid_aria_role =
                        Some(biome_config::RuleFixConfiguration::WithOptions(
                            biome_config::RuleWithFixOptions {
                                level: severity.into(),
                                fix: None,
                                options: *Box::new((*rule_options).into()),
                            },
                        ));
                }
            }
        }
        eslint_eslint::Rule::TypeScriptArrayType(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results)
                && let eslint_eslint::RuleConf::Option(severity, rule_options) = conf
            {
                let group = rules.style.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
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
        eslint_eslint::Rule::TypeScriptConsistentTypeImports(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results)
                && let eslint_eslint::RuleConf::Option(severity, rule_options) = conf
            {
                let group = rules.style.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
                    group.use_import_type = Some(biome_config::RuleFixConfiguration::WithOptions(
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
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results)
                && let eslint_eslint::RuleConf::Option(severity, rule_options) = conf
            {
                let group = rules.style.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
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
                if let SeverityOrGroup::Group(group) = group {
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
        }
        eslint_eslint::Rule::UnicornFilenameCase(conf) => {
            if migrate_eslint_any_rule(rules, &name, conf.severity(), opts, results) {
                let group = rules.style.get_or_insert_with(Default::default);
                if let SeverityOrGroup::Group(group) = group {
                    group.use_filenaming_convention =
                        Some(biome_config::RuleConfiguration::WithOptions(
                            biome_config::RuleWithOptions {
                                level: conf.severity().into(),
                                options: conf.option_or_default().into(),
                            },
                        ));
                }
            }
        }
    }
}

fn to_biome_includes(
    files: &[impl AsRef<str>],
    ignores: &[impl AsRef<str>],
) -> Vec<biome_glob::NormalizedGlob> {
    let mut includes: Vec<biome_glob::NormalizedGlob> = Vec::new();
    if !files.is_empty() {
        includes.extend(files.iter().filter_map(|glob| glob.as_ref().parse().ok()));
    }
    if !ignores.is_empty() {
        if includes.is_empty()
            && let Ok(glob) = "**".parse()
        {
            includes.push(glob);
        }
        includes.extend(ignores.iter().filter_map(|glob| {
            // ESLint supports negation: https://eslint.org/docs/latest/use/configure/ignore#unignoring-files-and-directories
            if let Some(rest) = glob.as_ref().strip_prefix('!') {
                rest.parse()
            } else {
                glob.as_ref()
                    .parse()
                    .map(|glob: biome_glob::NormalizedGlob| glob.negated())
            }
            .ok()
        }));
    }
    includes
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_configuration::OverrideGlobs;
    use eslint_eslint::*;
    use std::borrow::Cow;

    #[test]
    fn test_eslint_stylistic_rules_order() {
        assert!(ESLINT_STYLISTIC_RULES.is_sorted());
    }

    #[test]
    fn flat_config_single_config_object() {
        let flat_config = FlatConfigData(vec![FlatConfigObject {
            files: vec!["*.js".into()].into(),
            ignores: vec!["*.test.js".into()],
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
        assert!(biome_config.assist.is_none());
        let linter = biome_config.linter.unwrap();
        assert_eq!(
            linter.includes.unwrap(),
            ["*.js".parse().unwrap(), "!*.test.js".parse().unwrap()],
        );
        assert!(linter.rules.is_some());
    }

    #[test]
    fn flat_config_multiple_config_object() {
        let flat_config = FlatConfigData(vec![
            FlatConfigObject {
                files: vec![].into(),
                ignores: vec!["*.test.js".into()],
                language_options: None,
                rules: None,
            },
            FlatConfigObject {
                files: vec![].into(),
                ignores: vec![],
                language_options: None,
                rules: Some(Rules(
                    [Rule::Any(Cow::Borrowed("eqeqeq"), Severity::Error)]
                        .into_iter()
                        .collect(),
                )),
            },
            FlatConfigObject {
                files: vec![].into(),
                ignores: vec!["*.spec.js".into()],
                language_options: None,
                rules: None,
            },
            FlatConfigObject {
                files: vec!["*.ts".into()].into(),
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
        assert!(biome_config.assist.is_none());
        let linter = biome_config.linter.unwrap();
        assert_eq!(
            linter.includes.unwrap(),
            [
                "**".parse().unwrap(),
                "!*.test.js".parse().unwrap(),
                "!*.spec.js".parse().unwrap()
            ]
        );
        assert_eq!(
            linter
                .rules
                .unwrap()
                .suspicious
                .unwrap()
                .unwrap_group()
                .no_double_equals,
            Some(biome_config::RuleFixConfiguration::Plain(
                biome_config::RulePlainConfiguration::Error
            ))
        );
        let overrides = biome_config.overrides.unwrap();
        assert_eq!(overrides.0.len(), 1);
        let override0 = overrides.0.into_iter().next().unwrap();
        assert_eq!(
            override0.includes.unwrap(),
            OverrideGlobs::Globs(["*.ts".parse().unwrap()].into_iter().collect()),
        );
        assert_eq!(
            override0
                .linter
                .unwrap()
                .rules
                .unwrap()
                .suspicious
                .unwrap()
                .unwrap_group()
                .no_double_equals,
            Some(biome_config::RuleFixConfiguration::Plain(
                biome_config::RulePlainConfiguration::Off
            ))
        );
    }
}
