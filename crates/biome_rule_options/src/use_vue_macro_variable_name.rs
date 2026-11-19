use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueMacroVariableNameOptions {
    /// The required name of the variable that stores the result of `defineProps()`.
    /// Defaults to `props`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub define_props: Option<Box<str>>,

    /// The required name of the variable that stores the result of `defineEmits()`.
    /// Defaults to `emit`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub define_emits: Option<Box<str>>,

    /// The required name of the variable that stores the result of `defineSlots()`.
    /// Defaults to `slots`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub define_slots: Option<Box<str>>,

    /// The required name of the variable that stores the result of `useSlots()`.
    /// Defaults to `slots`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub use_slots: Option<Box<str>>,

    /// The required name of the variable that stores the result of `useAttrs()`.
    /// Defaults to `attrs`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub use_attrs: Option<Box<str>>,
}

impl UseVueMacroVariableNameOptions {
    pub const DEFAULT_DEFINE_PROPS: &'static str = "props";
    pub const DEFAULT_DEFINE_EMITS: &'static str = "emit";
    pub const DEFAULT_DEFINE_SLOTS: &'static str = "slots";
    pub const DEFAULT_USE_SLOTS: &'static str = "slots";
    pub const DEFAULT_USE_ATTRS: &'static str = "attrs";

    /// Returns [`Self::define_props`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_DEFINE_PROPS`].
    pub fn define_props(&self) -> &str {
        self.define_props
            .as_deref()
            .unwrap_or(Self::DEFAULT_DEFINE_PROPS)
    }

    /// Returns [`Self::define_emits`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_DEFINE_EMITS`].
    pub fn define_emits(&self) -> &str {
        self.define_emits
            .as_deref()
            .unwrap_or(Self::DEFAULT_DEFINE_EMITS)
    }

    /// Returns [`Self::define_slots`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_DEFINE_SLOTS`].
    pub fn define_slots(&self) -> &str {
        self.define_slots
            .as_deref()
            .unwrap_or(Self::DEFAULT_DEFINE_SLOTS)
    }

    /// Returns [`Self::use_slots`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_USE_SLOTS`].
    pub fn use_slots(&self) -> &str {
        self.use_slots.as_deref().unwrap_or(Self::DEFAULT_USE_SLOTS)
    }

    /// Returns [`Self::use_attrs`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_USE_ATTRS`].
    pub fn use_attrs(&self) -> &str {
        self.use_attrs.as_deref().unwrap_or(Self::DEFAULT_USE_ATTRS)
    }
}
