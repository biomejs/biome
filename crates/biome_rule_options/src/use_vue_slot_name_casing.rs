use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueSlotNameCasingOptions {
    /// The casing that slot names must follow. If omitted, slot names must be in camelCase.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub casing: Option<SlotNameCasing>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum SlotNameCasing {
    /// Slot names must be in camelCase, e.g. `fooBar`.
    #[default]
    #[serde(rename = "camelCase")]
    Camel,
    /// Slot names must be in kebab-case, e.g. `foo-bar`.
    #[serde(rename = "kebab-case")]
    Kebab,
    /// Slot names must only contain lowercase letters from `a` to `z`, e.g. `foo`.
    #[serde(rename = "singleword")]
    SingleWord,
}

impl UseVueSlotNameCasingOptions {
    pub const DEFAULT_CASING: SlotNameCasing = SlotNameCasing::Camel;

    /// Returns [`Self::casing`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CASING`].
    pub fn casing(&self) -> SlotNameCasing {
        self.casing.unwrap_or(Self::DEFAULT_CASING)
    }
}
