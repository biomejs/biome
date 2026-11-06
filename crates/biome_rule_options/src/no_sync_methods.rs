use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `noSyncMethods` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoSyncMethodsOptions {
    /// Allows synchronous calls at the top level of a file, outside of any function.
    ///
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_at_root_level: Option<bool>,

    /// Names of synchronous functions that the rule doesn't report, such as `readFileSync`.
    ///
    /// Defaults to `[]`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

impl NoSyncMethodsOptions {
    pub const DEFAULT_ALLOW_AT_ROOT_LEVEL: bool = false;

    /// Returns [`Self::allow_at_root_level`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_AT_ROOT_LEVEL`].
    pub fn allow_at_root_level(&self) -> bool {
        self.allow_at_root_level
            .unwrap_or(Self::DEFAULT_ALLOW_AT_ROOT_LEVEL)
    }

    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }
}
