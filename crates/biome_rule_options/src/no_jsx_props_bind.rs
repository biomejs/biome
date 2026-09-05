use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoJsxPropsBindOptions {
    /// Whether to allow arrow functions in JSX props. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_arrow_functions: Option<bool>,
    /// Whether to allow function expressions in JSX props. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_functions: Option<bool>,
    /// Whether to allow `.bind()` calls in JSX props. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_bind: Option<bool>,
    /// Whether to ignore DOM components (`<div>`, `<span>`, etc.). Defaults to `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_dom_components: Option<bool>,
    /// Whether to ignore `ref` props. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_refs: Option<bool>,
}

impl NoJsxPropsBindOptions {
    pub fn allow_arrow_functions(&self) -> bool {
        self.allow_arrow_functions.unwrap_or(false)
    }
    pub fn allow_functions(&self) -> bool {
        self.allow_functions.unwrap_or(false)
    }
    pub fn allow_bind(&self) -> bool {
        self.allow_bind.unwrap_or(false)
    }
    pub fn ignore_dom_components(&self) -> bool {
        self.ignore_dom_components.unwrap_or(false)
    }
    pub fn ignore_refs(&self) -> bool {
        self.ignore_refs.unwrap_or(false)
    }
}
