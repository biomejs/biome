use biome_analyze::RuleMetadata;

use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Explain {
    Rule(RuleMetadata),
    Unknown(String),
}

impl FromStr for Explain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Explain::Unknown(s.to_owned()))
    }
}
