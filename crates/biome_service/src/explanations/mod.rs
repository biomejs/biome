mod rules;
use biome_analyze::RuleMetadata;
use rules::get_rule_metadata;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Explanations {
    pub rule: Option<RuleMetadata>,
}

impl FromStr for Explanations {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Explanations {
            rule: get_rule_metadata(s),
        })
    }
}
