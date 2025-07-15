use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
/// Options for the useJsxKeyInIterable rule, which applies to both React and Qwik.
pub struct UseJsxKeyInIterableOptions {
    /// Set to `true` to check shorthand fragments (`<></>`) (React only)
    pub check_shorthand_fragments: bool,
}
