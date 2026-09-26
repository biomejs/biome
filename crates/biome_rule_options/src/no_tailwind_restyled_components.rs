use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTailwindRestyledComponentsOptions {
    /// Additional component-specific exceptions. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Box<[TailwindComponentAllowance]>>,
}

impl NoTailwindRestyledComponentsOptions {
    /// Returns [`Self::allow`] if it is set.
    /// Otherwise, returns an empty list.
    pub fn allow(&self) -> &[TailwindComponentAllowance] {
        self.allow.as_deref().unwrap_or_default()
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TailwindComponentAllowance {
    /// An exact component name, an array of names, or `"*"` for all components.
    #[deserializable(required)]
    pub components: TailwindAllowedComponents,
    /// Categories allowed on the matching components. Defaults to an empty list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub categories: Vec<TailwindAppearanceCategory>,
    /// Exact classes, including variants and modifiers. Defaults to an empty list.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub classes: Vec<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum TailwindAllowedComponents {
    Name(Box<str>),
    Names(Box<[Box<str>]>),
}

impl Default for TailwindAllowedComponents {
    fn default() -> Self {
        Self::Names(Box::default())
    }
}

impl Deserializable for TailwindAllowedComponents {
    fn deserialize(
        ctx: &mut dyn DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type()? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Name)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Names)
        }
    }
}

impl TailwindAllowedComponents {
    pub fn matches(&self, matches_name: impl Fn(&str) -> bool) -> bool {
        match self {
            Self::Name(name) => name.as_ref() == "*" || matches_name(name),
            Self::Names(names) => names.iter().any(|name| matches_name(name)),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TailwindAppearanceCategory {
    Color,
    Typography,
    Spacing,
    Shape,
    Effects,
    Motion,
}
