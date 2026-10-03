use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentJsonFileReadOptions {
    /// Whether JSON files should be read as strings or as `Buffer`s before they are passed to `JSON.parse()`.
    ///
    /// Default: `"string"`
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub read_as: Option<JsonFileReadAs>,
}

impl UseConsistentJsonFileReadOptions {
    pub const DEFAULT_READ_AS: JsonFileReadAs = JsonFileReadAs::String;

    /// Returns [`Self::read_as`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_READ_AS`].
    pub fn read_as(&self) -> JsonFileReadAs {
        self.read_as.unwrap_or(Self::DEFAULT_READ_AS)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum JsonFileReadAs {
    /// Read JSON files with the `'utf8'` encoding, so `JSON.parse()` receives a string.
    #[default]
    String,
    /// Read JSON files without an encoding, so `JSON.parse()` receives a `Buffer`.
    Buffer,
}
