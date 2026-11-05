use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueRestrictedComponentOptionsOptions {
    /// Component options that must not be used. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component_options: Option<Box<[RestrictedComponentOption]>>,
}

impl NoVueRestrictedComponentOptionsOptions {
    /// Returns [`Self::component_options`] if it is set.
    /// Otherwise, returns an empty list.
    pub fn component_options(&self) -> &[RestrictedComponentOption] {
        self.component_options.as_deref().unwrap_or_default()
    }
}

/// A component option that must not be used: either its path alone, or its
/// path with a custom message.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RestrictedComponentOption {
    Path(ComponentOptionPath),
    WithMessage(RestrictedComponentOptionWithMessage),
}

impl RestrictedComponentOption {
    /// Returns the names of the properties that lead to the option, starting
    /// with a property of the component options object.
    pub fn names(&self) -> &[Box<str>] {
        match self {
            Self::Path(path) => path.names(),
            Self::WithMessage(option) => option.name.names(),
        }
    }

    /// Returns the custom message of the option, if it has one.
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Path(_) => None,
            Self::WithMessage(option) => option.message.as_deref(),
        }
    }
}

impl Deserializable for RestrictedComponentOption {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type(ctx)? == DeserializableType::Map {
            Deserializable::deserialize(ctx, value, name).map(Self::WithMessage)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Path)
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedComponentOptionWithMessage {
    /// The name of the option, or the list of property names that leads to it.
    #[deserializable(required)]
    pub name: ComponentOptionPath,
    /// The message to show in place of the default message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<str>>,
}

/// The name of a component option, or the list of property names that leads
/// to it. The name `*` matches any property.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ComponentOptionPath {
    Name(Box<str>),
    Names(Box<[Box<str>]>),
}

impl ComponentOptionPath {
    /// The name that matches any property.
    pub const WILDCARD: &str = "*";

    /// Returns the names of the properties that lead to the option.
    pub fn names(&self) -> &[Box<str>] {
        match self {
            Self::Name(name) => std::slice::from_ref(name),
            Self::Names(names) => names,
        }
    }
}

impl Default for ComponentOptionPath {
    fn default() -> Self {
        Self::Names(Box::default())
    }
}

impl Deserializable for ComponentOptionPath {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type(ctx)? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Name)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::Names)
        }
    }
}
