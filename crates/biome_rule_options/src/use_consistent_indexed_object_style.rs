use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentIndexedObjectStyleOptions {
    /// The style that object types with arbitrary keys must use.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<IndexedObjectStyle>,
}

impl UseConsistentIndexedObjectStyleOptions {
    pub const DEFAULT_STYLE: IndexedObjectStyle = IndexedObjectStyle::Record;

    /// Returns [`Self::style`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_STYLE`].
    pub fn style(&self) -> IndexedObjectStyle {
        self.style.unwrap_or(Self::DEFAULT_STYLE)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum IndexedObjectStyle {
    /// Prefer the `Record` type: `Record<string, unknown>`
    #[default]
    Record,
    /// Prefer index signatures: `{ [key: string]: unknown }`
    IndexSignature,
}
