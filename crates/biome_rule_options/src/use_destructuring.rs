use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseDestructuringOptions {
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub variable_declarator: Option<DestructuringConfig>,

    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub assignment_expression: Option<DestructuringConfig>,
}

impl UseDestructuringOptions {
    pub fn variable_declarator(&self) -> &DestructuringConfig {
        static DEFAULT: DestructuringConfig = DestructuringConfig::new();
        self.variable_declarator.as_ref().unwrap_or(&DEFAULT)
    }

    pub fn assignment_expression(&self) -> &DestructuringConfig {
        static DEFAULT: DestructuringConfig = DestructuringConfig::new();
        self.assignment_expression.as_ref().unwrap_or(&DEFAULT)
    }
}

#[derive(Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct DestructuringConfig {
    pub array: bool,
    pub object: bool,
}

impl Default for DestructuringConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DestructuringConfig {
    const fn new() -> Self {
        Self {
            array: true,
            object: true,
        }
    }
}
