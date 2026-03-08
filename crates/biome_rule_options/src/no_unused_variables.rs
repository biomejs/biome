use regex::Regex;
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnusedVariablesOptions {
    /// Whether to ignore unused variables from an object destructuring with a spread.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_rest_siblings: Option<bool>,

    /// A Regex pattern to ignore matching variables from this rule with.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_pattern: Option<Box<str>>,
}

impl NoUnusedVariablesOptions {
    pub const DEFAULT_IGNORE_REST_SIBLINGS: bool = true;

    /// Returns [`Self::ignore_rest_siblings`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_REST_SIBLINGS`].
    pub fn ignore_rest_siblings(&self) -> bool {
        self.ignore_rest_siblings
            .unwrap_or(Self::DEFAULT_IGNORE_REST_SIBLINGS)
    }

    // TODO see if we can cache the compilation of the regex
    // TODO see if proper errors are thrown on invalid regex
    pub fn ignore_pattern(&self) -> Regex {
        let pattern = self.ignore_pattern.clone().unwrap_or(Box::from("$-"));
        Regex::new(&pattern).unwrap()
    }
}
