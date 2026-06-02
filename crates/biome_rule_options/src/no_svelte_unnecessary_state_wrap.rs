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
