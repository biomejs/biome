use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueChildContentOptions {
    /// Names of other directives that replace the content of their element, written without
    /// the `v-` prefix. For example, `"t"` checks the `v-t` directive. `v-html` and `v-text`
    /// are always checked. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_directives: Option<Box<[Box<str>]>>,
}

impl NoVueChildContentOptions {
    /// Returns [`Self::additional_directives`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn additional_directives(&self) -> &[Box<str>] {
        self.additional_directives.as_deref().unwrap_or_default()
    }
}
