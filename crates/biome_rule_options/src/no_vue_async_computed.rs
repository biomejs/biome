use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueAsyncComputedOptions {
    /// Names of objects whose `then()`, `catch()`, and `finally()` methods aren't treated as
    /// asynchronous, such as `z` from the Zod validation library.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_objects: Option<Box<[Box<str>]>>,
}

impl NoVueAsyncComputedOptions {
    /// Returns [`Self::ignore_objects`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignore_objects(&self) -> &[Box<str>] {
        self.ignore_objects.as_deref().unwrap_or_default()
    }
}
