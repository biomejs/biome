use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoBlankTargetOptions {
    /// List of domains where `target="_blank"` is allowed without
    /// `rel="noopener"`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_domains: Vec<String>,

    /// Whether `noreferrer` is allowed in addition to `noopener`.
    #[serde(default = "default_allow_no_referrer")]
    pub allow_no_referrer: bool,
}

impl Default for NoBlankTargetOptions {
    fn default() -> Self {
        Self {
            allow_domains: Default::default(),
            allow_no_referrer: default_allow_no_referrer(),
        }
    }
}

fn default_allow_no_referrer() -> bool {
    true
}
