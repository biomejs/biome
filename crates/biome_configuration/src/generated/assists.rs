//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::analyzer::assists::*;
use biome_analyze::{AnalyzerRules, MetadataRegistry};
pub fn push_to_analyzer_assists(
    rules: &Rules,
    metadata: &MetadataRegistry,
    analyzer_rules: &mut AnalyzerRules,
) {
    if let Some(rules) = rules.refactor.as_ref() {
        for rule_name in Refactor::GROUP_RULES {
            if let Some((_, Some(rule_options))) = rules.get_rule_configuration(rule_name) {
                if let Some(rule_key) = metadata.find_rule("refactor", rule_name) {
                    analyzer_rules.push_rule(rule_key, rule_options);
                }
            }
        }
    }
}
