use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseValidAriaRoleOptions {
    /// It allows specifying a list of roles that might be invalid otherwise
    pub allow_invalid_roles: Box<[Box<str>]>,
    /// Use this option to ignore non-DOM elements, such as custom components
    pub ignore_non_dom: bool,
}
