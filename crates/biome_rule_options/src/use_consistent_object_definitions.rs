use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentObjectDefinitionsOptions {
    /// The preferred syntax to enforce.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub syntax: Option<ObjectPropertySyntax>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub enum ObjectPropertySyntax {
    /// `{foo: foo}`
    Explicit,
    /// `{foo}`
    #[default]
    Shorthand,
}
