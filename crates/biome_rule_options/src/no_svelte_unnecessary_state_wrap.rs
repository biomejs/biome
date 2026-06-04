use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoSvelteUnnecessaryStateWrapOptions {
    /// Additional class names to treat as already reactive (beyond the built-in `svelte/reactivity` classes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_reactive_classes: Option<Box<[Box<str>]>>,

    /// When `true`, allows `$state()` wrapping for variables that are reassigned after declaration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reassign: Option<bool>,
}

impl NoSvelteUnnecessaryStateWrapOptions {
    pub const DEFAULT_ALLOW_REASSIGN: bool = false;

    /// Returns [`Self::allow_reassign`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_REASSIGN`].
    pub fn allow_reassign(&self) -> bool {
        self.allow_reassign.unwrap_or(Self::DEFAULT_ALLOW_REASSIGN)
    }
}
