mod rules;
use biome_analyze::RuleMetadata;
use rules::get_rule_metadata;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Explain {
    pub rule: Option<RuleMetadata>,
    pub unknown: Option<String>,
}

impl FromStr for Explain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rule = get_rule_metadata(s);

        let unknown = if rule.is_none() {
            Some(s.to_owned())
        } else {
            None
        };

        Ok(Explain { rule, unknown })
    }
}
