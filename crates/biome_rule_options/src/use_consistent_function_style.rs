use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Configures the required function style and whether declaration mode permits arrow functions.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentFunctionStyleOptions {
    /// The function style to enforce. Default: `"expression"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<FunctionStyle>,
    /// Allow arrow functions when declarations are required. Default: `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_arrow_functions: Option<bool>,
}

impl UseConsistentFunctionStyleOptions {
    pub const DEFAULT_ALLOW_ARROW_FUNCTIONS: bool = false;

    /// Returns [`Self::style`] if it is set.
    /// Otherwise, returns [`FunctionStyle::default()`].
    pub fn style(&self) -> FunctionStyle {
        self.style.unwrap_or_default()
    }

    /// Returns [`Self::allow_arrow_functions`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_ALLOW_ARROW_FUNCTIONS`].
    pub fn allow_arrow_functions(&self) -> bool {
        self.allow_arrow_functions
            .unwrap_or(Self::DEFAULT_ALLOW_ARROW_FUNCTIONS)
    }
}

/// The required form for function definitions: `"expression"` or `"declaration"`.
#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum FunctionStyle {
    /// Require function expressions assigned to variables. This is the default.
    #[default]
    Expression,
    /// Require function declarations, subject to the rule's exceptions.
    Declaration,
}
