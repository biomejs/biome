use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseReadonlyClassPropertiesOptions {
    /// When `true`, the keywords `public`, `protected`, and `private` are analyzed by the rule.
    #[serde(default, skip_serializing_if = "is_default")]
    pub check_all_properties: bool,
}

fn is_default<T: Default + Eq>(value: &T) -> bool {
    value == &T::default()
}
