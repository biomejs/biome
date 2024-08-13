//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::linter::*;
use biome_analyze::{AnalyzerRules, MetadataRegistry};
pub fn push_to_analyzer_rules(
    rules: &Rules,
    metadata: &MetadataRegistry,
    analyzer_rules: &mut AnalyzerRules,
) {
    if let Some(rules) = rules.a11y.as_ref() {
        for rule_name in A11y::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("a11y", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.complexity.as_ref() {
        for rule_name in Complexity::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("complexity", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.correctness.as_ref() {
        for rule_name in Correctness::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("correctness", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.nursery.as_ref() {
        for rule_name in Nursery::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("nursery", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.performance.as_ref() {
        for rule_name in Performance::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("performance", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.security.as_ref() {
        for rule_name in Security::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("security", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.style.as_ref() {
        for rule_name in Style::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("style", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
    if let Some(rules) = rules.suspicious.as_ref() {
        for rule_name in Suspicious::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("suspicious", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
}
