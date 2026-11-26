use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueRegisteredComponentNameCasingOptions {
    /// The casing that the names of components registered in the `components` option must
    /// follow. If omitted, names must be in PascalCase.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub casing: Option<RegisteredComponentNameCasing>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum RegisteredComponentNameCasing {
    /// Component names must be in PascalCase, e.g. `TodoItem`.
    #[default]
    #[serde(rename = "PascalCase")]
    Pascal,
    /// Component names must be in camelCase, e.g. `todoItem`.
    #[serde(rename = "camelCase")]
    Camel,
    /// Component names must be in kebab-case, e.g. `todo-item`.
    #[serde(rename = "kebab-case")]
    Kebab,
}

impl UseVueRegisteredComponentNameCasingOptions {
    pub const DEFAULT_CASING: RegisteredComponentNameCasing = RegisteredComponentNameCasing::Pascal;

    /// Returns [`Self::casing`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CASING`].
    pub fn casing(&self) -> RegisteredComponentNameCasing {
        self.casing.unwrap_or(Self::DEFAULT_CASING)
    }
}
