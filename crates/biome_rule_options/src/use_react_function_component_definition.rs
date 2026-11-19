use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseReactFunctionComponentDefinitionOptions {
    /// The function style to enforce for named React components.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub named_components: Option<ComponentDefinitionStyle>,
}

#[derive(
    Clone, Copy, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize, Default,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum ComponentDefinitionStyle {
    /// Enforce function declarations.
    #[default]
    FunctionDeclaration,
    /// Enforce function expressions assigned to a component binding.
    FunctionExpression,
    /// Enforce arrow functions assigned to a component binding.
    ArrowFunction,
}

impl ComponentDefinitionStyle {
    pub const fn label(self) -> &'static str {
        match self {
            Self::FunctionDeclaration => "a function declaration",
            Self::FunctionExpression => "a function expression",
            Self::ArrowFunction => "an arrow function",
        }
    }
}
