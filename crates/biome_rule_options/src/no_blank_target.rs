use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoBlankTargetOptions {
    /// List of domains where `target="_blank"` is allowed without
    /// `rel="noopener"`.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub allow_domains: Vec<String>,

    /// Whether `noreferrer` is allowed in addition to `noopener`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_no_referrer: Option<bool>,
}

impl NoBlankTargetOptions {
    pub const DEFAULT_ALLOW_NO_REFERRER: bool = true;

    /// Returns [`Self::allow_no_referrer`] if it is set
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_NO_REFERRER`].
    pub fn allow_no_referrer(&self) -> bool {
        self.allow_no_referrer
            .unwrap_or(Self::DEFAULT_ALLOW_NO_REFERRER)
    }
}
