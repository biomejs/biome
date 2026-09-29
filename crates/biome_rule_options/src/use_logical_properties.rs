use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseLogicalPropertiesOptions {
    /// The text direction used to map physical inline properties. Defaults to `"ltr"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub direction: Option<UseLogicalPropertiesDirection>,
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase")]
pub enum UseLogicalPropertiesDirection {
    #[default]
    Ltr,
    Rtl,
}

impl UseLogicalPropertiesOptions {
    pub const DEFAULT_DIRECTION: UseLogicalPropertiesDirection = UseLogicalPropertiesDirection::Ltr;

    pub fn direction(&self) -> UseLogicalPropertiesDirection {
        self.direction.unwrap_or(Self::DEFAULT_DIRECTION)
    }
}
