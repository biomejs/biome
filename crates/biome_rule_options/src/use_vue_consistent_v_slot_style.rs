use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentVSlotStyleOptions {
    /// Preferred style for the default slot when `v-slot` is placed directly on a component,
    /// as in `<MyComponent v-slot="props">`.
    /// If omitted, `"v-slot"` is preferred.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub at_component: Option<VueSlotStyle>,

    /// Preferred style for the default slot on a `<template>` element,
    /// as in `<template #default>`.
    /// If omitted, `"shorthand"` is preferred.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub default: Option<VueSlotStyle>,

    /// Preferred style for named slots, as in `<template #header>`.
    /// If omitted, `"shorthand"` is preferred.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub named: Option<VueNamedSlotStyle>,
}

/// How a default slot is written.
#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VueSlotStyle {
    /// `#default`
    #[default]
    Shorthand,
    /// `v-slot:default`
    Longhand,
    /// `v-slot`, without a slot name.
    #[serde(rename = "v-slot")]
    VSlot,
}

/// How a named slot is written.
#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VueNamedSlotStyle {
    /// `#name`
    #[default]
    Shorthand,
    /// `v-slot:name`
    Longhand,
}

impl From<VueNamedSlotStyle> for VueSlotStyle {
    fn from(style: VueNamedSlotStyle) -> Self {
        match style {
            VueNamedSlotStyle::Shorthand => Self::Shorthand,
            VueNamedSlotStyle::Longhand => Self::Longhand,
        }
    }
}

impl UseVueConsistentVSlotStyleOptions {
    pub const DEFAULT_AT_COMPONENT: VueSlotStyle = VueSlotStyle::VSlot;
    pub const DEFAULT_DEFAULT: VueSlotStyle = VueSlotStyle::Shorthand;
    pub const DEFAULT_NAMED: VueNamedSlotStyle = VueNamedSlotStyle::Shorthand;

    /// Returns [`Self::at_component`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_AT_COMPONENT`].
    pub fn at_component(&self) -> VueSlotStyle {
        self.at_component.unwrap_or(Self::DEFAULT_AT_COMPONENT)
    }

    /// Returns the `default` field if it is set.
    /// Otherwise, returns [`Self::DEFAULT_DEFAULT`].
    pub fn default_slot(&self) -> VueSlotStyle {
        self.default.unwrap_or(Self::DEFAULT_DEFAULT)
    }

    /// Returns [`Self::named`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_NAMED`].
    pub fn named(&self) -> VueNamedSlotStyle {
        self.named.unwrap_or(Self::DEFAULT_NAMED)
    }
}
