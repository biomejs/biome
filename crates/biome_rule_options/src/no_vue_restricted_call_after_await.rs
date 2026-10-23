use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueRestrictedCallAfterAwaitOptions {
    /// Functions that must not be called after an `await` in `setup()`. Defaults to an empty list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Box<[RestrictedFunction]>>,
}

impl NoVueRestrictedCallAfterAwaitOptions {
    /// Returns [`Self::functions`] if it is set.
    /// Otherwise, returns an empty list.
    pub fn functions(&self) -> &[RestrictedFunction] {
        self.functions.as_deref().unwrap_or_default()
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RestrictedFunction {
    /// The module that exports the function, written as it appears in the `import` statement.
    #[deserializable(required)]
    pub module: Box<str>,
    /// The name the function is exported as, or a list of names that leads to the function
    /// through the module's exports. Defaults to `"default"`, the module's default export.
    #[serde(default, skip_serializing_if = "RestrictedFunctionPath::is_default")]
    pub path: RestrictedFunctionPath,
    /// The message to show in place of the default message.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<str>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum RestrictedFunctionPath {
    Name(Box<str>),
    Names(Box<[Box<str>]>),
}

impl RestrictedFunctionPath {
    pub const DEFAULT_NAME: &str = "default";

    /// Returns the names that lead to the function, starting with the export of the module.
    pub fn names(&self) -> &[Box<str>] {
        match self {
            Self::Name(name) => std::slice::from_ref(name),
            Self::Names(names) => names,
        }
    }

    fn is_default(&self) -> bool {
        matches!(self.names(), [name] if name.as_ref() == Self::DEFAULT_NAME)
    }
}

impl Default for RestrictedFunctionPath {
    fn default() -> Self {
        Self::Name(Self::DEFAULT_NAME.into())
    }
}

impl Deserializable for RestrictedFunctionPath {
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
