use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoParameterAssignOptions {
    /// Whether to report an error when a dependency is listed in the dependencies array but isn't used. Defaults to `allow`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub property_assignment: Option<PropertyAssignmentMode>,
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
/// Specifies whether property assignments on function parameters are allowed or denied.
pub enum PropertyAssignmentMode {
    /// Allows property assignments on function parameters.
    /// This is the default behavior, enabling flexibility in parameter usage.
    #[default]
    Allow,

    /// Disallows property assignments on function parameters.
    /// Enforces stricter immutability to prevent unintended side effects.
    Deny,
}
impl biome_deserialize::Merge for PropertyAssignmentMode {
    fn merge_with(&mut self, other: Self) {
        *self = other;
    }
}
