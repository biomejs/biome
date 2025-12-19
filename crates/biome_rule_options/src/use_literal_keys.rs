use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseLiteralKeysOptions {
    /// When `true`, properties accessed via index signatures must use bracket notation.
    /// This mirrors TypeScript's `noPropertyAccessFromIndexSignature` compiler option.
    #[serde(default)]
    pub no_property_access_from_index_signature: bool,
}
