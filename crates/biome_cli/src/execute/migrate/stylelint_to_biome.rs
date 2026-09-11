//! This module includes implementations for converting a Stylelint config to a Biome config.
//!
//! The conversion relies on the generated
//! [super::stylelint_any_rule_to_biome::migrate_stylelint_any_rule] function that uses
//! Biome's rule metadata to determine the equivalent Biome rule of a Stylelint rule.

use std::collections::{BTreeMap, BTreeSet};

use super::eslint_to_biome::{UnsupportedRuleReason, to_biome_includes};
use super::stylelint_any_rule_to_biome::migrate_stylelint_any_rule;
use super::stylelint_stylelint::{self, Severity};
use biome_configuration::analyzer::presets::PresetConfig;
use biome_configuration::{self as biome_config};
use biome_console::markup;
use biome_deserialize::Merge;
use biome_diagnostics::Location;

// The per-rule outcome and migration options are shared with the ESLint migration.
pub(crate) use super::eslint_to_biome::{MigrationOptions, RuleMigrationResult};

#[derive(Debug, Default)]
pub(crate) struct MigrationResults {
    /// Path to the migrated Stylelint configuration.
    pub(crate) stylelint_path: Option<Box<str>>,
    /// Is the Biome configuration updated?
    pub(crate) write: bool,
    /// Inspired rules that were not migrated because `include_inspired` is disabled.
    pub(crate) inspired: BTreeSet<Box<str>>,
    /// Nursery rules that were not migrated because `include_nursery` is disabled.
    pub(crate) nursery: BTreeSet<Box<str>>,
    /// Rules that have been migrated.
    pub(crate) migrated: BTreeSet<Box<str>>,
    /// Rules that don't have a Biome equivalent.
    pub(crate) unsupported: BTreeMap<Box<str>, UnsupportedRuleReason>,
}
impl MigrationResults {
    pub(crate) fn add(&mut self, rule_name: &str, status: RuleMigrationResult) {
        match status {
            RuleMigrationResult::Migrated => {
                self.migrated.insert(rule_name.into());
            }
            RuleMigrationResult::Inspired => {
                self.inspired.insert(rule_name.into());
            }
            RuleMigrationResult::Nursery => {
                self.nursery.insert(rule_name.into());
            }
            RuleMigrationResult::Unsupported => {
                self.unsupported
                    .insert(rule_name.into(), unsupported_rule_reason(rule_name));
            }
        }
    }

    pub(crate) fn rule_count(&self) -> usize {
        self.migrated.len() + self.inspired.len() + self.nursery.len() + self.unsupported.len()
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
        if let Some(path) = self.stylelint_path.as_ref() {
            builder = builder.resource(path);
        }
        builder.build()
    }

    fn message(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        let count = self.rule_count();
        if count == 0 {
            return fmt.write_markup(markup! { "No rules to migrate." });
        }
        let formatter_covers_count = self
            .unsupported
            .values()
            .filter(|reason| {
                matches!(
                    reason,
                    UnsupportedRuleReason::FormatterCovers
                        | UnsupportedRuleReason::FormatterOption(_)
                )
            })
            .count();

        let directly_covered_count = self.migrated.len();
        let inspired_count = self.inspired.len();
        let nursery_count = self.nursery.len();

        let total_migratable_count = directly_covered_count + inspired_count + nursery_count;
        let total_covered_count = total_migratable_count + formatter_covers_count;
        let total_covered_percent = (total_covered_count * 100).checked_div(count).unwrap_or(0);
        let directly_covered_percent = (directly_covered_count * 100)
            .checked_div(count)
            .unwrap_or(0);

        fmt.write_markup(markup! { <Emphasis>{count}" Stylelint rules found\n"</Emphasis> })?;
        if formatter_covers_count > 0 {
            fmt.write_markup(markup! { "- "<Emphasis><Success>{formatter_covers_count}</Success>" are obsolete"</Emphasis>" because of Biome's formatter\n" })?;
        }

        if self.write {
            fmt.write_markup(markup! { "- "<Emphasis><Success>{directly_covered_count}</Success>" have been migrated"</Emphasis>" to Biome's rules\n" })?;
        } else {
            fmt.write_markup(markup! { "- "<Emphasis><Success>{directly_covered_count}</Success>" can be migrated"</Emphasis>" to Biome's rules (run with --write to migrate)\n" })?;
            if inspired_count > 0 {
                fmt.write_markup(markup! { "  - "<Emphasis><Success>"+"{inspired_count}</Success></Emphasis>" with --include-inspired\n" })?;
            }
            if nursery_count > 0 {
                fmt.write_markup(markup! { "  - "<Emphasis><Success>"+"{nursery_count}</Success></Emphasis>" with --include-nursery (experimental rules)\n" })?;
            }
        }

        fmt.write_markup(markup! {
            "- "<Emphasis><Success>{total_covered_percent}"% ("{total_covered_count}")"</Success>" of your Stylelint rules are fully covered by Biome\n"</Emphasis>
        })?;
        fmt.write_markup(markup! {
            "  - "{directly_covered_percent}"% ("{directly_covered_count}") via direct migration to Biome rules\n"
        })
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
            record_rule_list(visitor, &self.migrated)?;
        }
        if !self.inspired.is_empty() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &markup! { "Rules that can be migrated to an inspired rule using "<Emphasis>"--include-inspired"</Emphasis>":" },
            )?;
            record_rule_list(visitor, &self.inspired)?;
        }
        if !self.nursery.is_empty() {
            visitor.record_log(
                biome_diagnostics::LogCategory::Info,
                &markup! { "Rules that can be migrated to a nursery rule using "<Emphasis>"--include-nursery"</Emphasis>":" },
            )?;
            record_rule_list(visitor, &self.nursery)?;
        }
        if !self.unsupported.is_empty() {
            let mut formatter_covered = Vec::new();
            let mut not_implemented = Vec::new();
            for (rule, reason) in &self.unsupported {
                match reason {
                    UnsupportedRuleReason::FormatterCovers
                    | UnsupportedRuleReason::FormatterOption(_)
                    | UnsupportedRuleReason::Stylistic => formatter_covered.push(rule),
                    _ => not_implemented.push(rule),
                }
            }
            if !formatter_covered.is_empty() {
                visitor.record_log(
                    biome_diagnostics::LogCategory::Info,
                    &markup! { "These rules enforce styles handled by Biome's formatter (so you don't lose the functionality):" },
                )?;
                let list: Vec<_> = formatter_covered
                    .iter()
                    .map(|item| *item as &dyn biome_console::fmt::Display)
                    .collect();
                visitor.record_list(list.as_slice())?;
            }
            if !not_implemented.is_empty() {
                visitor.record_log(
                    biome_diagnostics::LogCategory::Info,
                    &markup! { "These rules don't have an equivalent Biome rule yet:" },
                )?;
                let list: Vec<_> = not_implemented
                    .iter()
                    .map(|item| *item as &dyn biome_console::fmt::Display)
                    .collect();
                visitor.record_list(list.as_slice())?;
            }
        }
        Ok(())
    }
}

fn record_rule_list(
    visitor: &mut dyn biome_diagnostics::Visit,
    rules: &BTreeSet<Box<str>>,
) -> std::io::Result<()> {
    let list: Vec<_> = rules
        .iter()
        .map(|item| item as &dyn biome_console::fmt::Display)
        .collect();
    visitor.record_list(list.as_slice())
}

/// Determines why an unmapped Stylelint rule is unsupported.
fn unsupported_rule_reason(rule_name: &str) -> UnsupportedRuleReason {
    if FORMATTER_COVERED_RULES.binary_search(&rule_name).is_ok() {
        UnsupportedRuleReason::FormatterCovers
    } else {
        UnsupportedRuleReason::KnownSourceNotImplemented
    }
}

/// Stylelint rules whose behavior is covered by Biome's formatter.
///
/// Most of these are stylistic rules that Stylelint deprecated in favor of a
/// dedicated formatter. They are kept here so the migration can report that the
/// functionality is not lost when using Biome's formatter.
///
/// Keep this list sorted so that [`slice::binary_search`] stays correct.
const FORMATTER_COVERED_RULES: &[&str] = &[];

pub(crate) fn merge_biome_config_with_stylelint(
    mut biome_config: biome_config::Configuration,
    stylelint_config: stylelint_stylelint::StylelintConfigData,
    options: &MigrationOptions,
) -> (biome_config::Configuration, MigrationResults) {
    let (stylelint_biome_config, results) = stylelint_config.into_biome_config(options);
    biome_config.merge_with(stylelint_biome_config);
    (biome_config, results)
}

impl stylelint_stylelint::StylelintConfigData {
    pub(crate) fn into_biome_config(
        self,
        options: &MigrationOptions,
    ) -> (biome_config::Configuration, MigrationResults) {
        let mut results = MigrationResults::default();
        let mut biome_config = biome_config::Configuration::default();
        let default_severity = self.default_severity.unwrap_or_default();
        let mut linter = biome_config::LinterConfiguration::default();
        let mut rules = self
            .rules
            .into_biome_rules(default_severity, options, &mut results);
        rules.preset = Some(PresetConfig::None);
        linter.rules = Some(rules);
        // `ignoreFiles` are already converted to negated Biome globs by
        // `IgnorePattern`, so they are added directly rather than through
        // `to_biome_includes`, which would negate them a second time.
        let mut includes: Vec<biome_glob::NormalizedGlob> = Vec::new();
        if !self.ignore_files.is_empty() {
            if let Ok(glob) = "**".parse() {
                includes.push(glob);
            }
            includes.extend(
                self.ignore_files
                    .iter()
                    .filter_map(|pattern| pattern.as_ref().parse().ok()),
            );
        }
        linter.includes = (!includes.is_empty()).then_some(includes);
        if !self.overrides.is_empty() {
            let mut overrides = biome_config::Overrides::default();
            for override_elt in self.overrides {
                let mut override_pattern = biome_config::OverridePattern::default();
                let includes = to_biome_includes(override_elt.files.as_slice(), &[] as &[&str]);
                override_pattern.includes = (!includes.is_empty())
                    .then_some(biome_config::OverrideGlobs::Globs(includes.into()));
                if !override_elt.rules.is_empty() {
                    let override_severity =
                        override_elt.default_severity.unwrap_or(default_severity);
                    override_pattern.linter = Some(biome_config::OverrideLinterConfiguration {
                        rules: Some(override_elt.rules.into_biome_rules(
                            override_severity,
                            options,
                            &mut results,
                        )),
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

impl stylelint_stylelint::Rules {
    pub(crate) fn into_biome_rules(
        self,
        default_severity: Severity,
        options: &MigrationOptions,
        results: &mut MigrationResults,
    ) -> biome_config::Rules {
        let mut rules = biome_config::Rules::default();
        for (name, data) in self.0 {
            if !data.enabled {
                continue;
            }
            let severity = data.severity.unwrap_or(default_severity);
            migrate_stylelint_any_rule(&mut rules, &name, severity, options, results);
        }
        rules
    }
}
