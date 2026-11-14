use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseTanstackQueryExhaustiveDependenciesOptions {
    /// Names of variables that the query function can use without adding them to the query
    /// key. This also covers the properties read from these variables.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignored_variables: Option<Box<[Box<str>]>>,

    /// Names of TypeScript types. The query function can use a variable or parameter declared
    /// with one of these types, such as `client: ApiClient`, without adding it to the query key.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignored_types: Option<Box<[Box<str>]>>,
}

impl UseTanstackQueryExhaustiveDependenciesOptions {
    /// Returns [`Self::ignored_variables`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignored_variables(&self) -> &[Box<str>] {
        self.ignored_variables.as_deref().unwrap_or_default()
    }

    /// Returns [`Self::ignored_types`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn ignored_types(&self) -> &[Box<str>] {
        self.ignored_types.as_deref().unwrap_or_default()
    }
}
