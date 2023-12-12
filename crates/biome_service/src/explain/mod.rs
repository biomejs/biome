mod rules;
use biome_analyze::RuleMetadata;
use rules::get_rule_metadata;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Explain {
    pub rule: Option<RuleMetadata>,
}

impl FromStr for Explain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Explain {
            rule: get_rule_metadata(s),
        })
    }
}
