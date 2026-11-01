use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveNestingDepthOptions {
    /// The maximum number of levels a rule or at-rule can be nested. Defaults to `3`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_depth: Option<u16>,

    /// At-rule names, without the `@`, that don't add a nesting level (case-insensitive).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore_at_rules: Vec<String>,
}

impl NoExcessiveNestingDepthOptions {
    pub const DEFAULT_MAX_DEPTH: u16 = 3;

    /// Returns [`Self::max_depth`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX_DEPTH`].
    pub fn max_depth(&self) -> u16 {
        self.max_depth.unwrap_or(Self::DEFAULT_MAX_DEPTH)
    }

    /// Whether the at-rule `name` is listed in [`Self::ignore_at_rules`].
    pub fn is_ignored_at_rule(&self, name: &str) -> bool {
        self.ignore_at_rules
            .iter()
            .any(|ignored| ignored.eq_ignore_ascii_case(name))
    }
}
