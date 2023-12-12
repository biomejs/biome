mod rules;
use biome_analyze::RuleMetadata;
use rules::get_rule_metadata;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Explain {
    Rule(RuleMetadata),
    Unknown(String),
}

impl FromStr for Explain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(metadata) = get_rule_metadata(s) {
            return Ok(Explain::Rule(metadata));
        }

        Ok(Explain::Unknown(s.to_owned()))
    }
}
