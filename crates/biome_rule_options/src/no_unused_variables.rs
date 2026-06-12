use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnusedVariablesOptions {
    /// Whether to ignore unused variables from an object destructuring with a spread.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_rest_siblings: Option<bool>,

    /// An object defining ignored identifiers for different language constructs.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<NoUnusedVariablesOptionsIgnore>,
}

impl NoUnusedVariablesOptions {
    pub const DEFAULT_IGNORE_REST_SIBLINGS: bool = true;

    /// Returns [`Self::ignore_rest_siblings`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_REST_SIBLINGS`].
    pub fn ignore_rest_siblings(&self) -> bool {
        self.ignore_rest_siblings
            .unwrap_or(Self::DEFAULT_IGNORE_REST_SIBLINGS)
    }

    /// Returns [`Self::ignore`] if it is set.
    /// Otherwise, returns an empty ignore object not matching anything.
    pub fn ignore(&self) -> NoUnusedVariablesOptionsIgnore {
        self.ignore.clone().unwrap_or_default()
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUnusedVariablesOptionsIgnore {
    /// An array of identifiers to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none", rename = "*")]
    pub all: Option<Box<[Box<str>]>>,

    /// An array of class names to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub class: Option<Box<[Box<str>]>>,

    /// An array of function names to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub function: Option<Box<[Box<str>]>>,

    /// An array of interface names to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub interface: Option<Box<[Box<str>]>>,

    /// An array of type aliases to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub type_alias: Option<Box<[Box<str>]>>,

    /// An array of type parameters to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub type_parameter: Option<Box<[Box<str>]>>,

    /// An array of variable names to ignore. Use "*" to ignore all identifiers.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub variable: Option<Box<[Box<str>]>>,
}

impl NoUnusedVariablesOptionsIgnore {
    pub const IGNORE_ALL: &'static str = "*";
}
