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
        static DEFAULT: DestructuringConfig = DestructuringConfig {
            array: None,
            object: None,
        };
        self.variable_declarator.as_ref().unwrap_or(&DEFAULT)
    }

    pub fn assignment_expression(&self) -> &DestructuringConfig {
        static DEFAULT: DestructuringConfig = DestructuringConfig {
            array: None,
            object: None,
        };
        self.assignment_expression.as_ref().unwrap_or(&DEFAULT)
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct DestructuringConfig {
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    array: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    object: Option<bool>,
}

impl DestructuringConfig {
    pub const DEFAULT_ARRAY: bool = true;
    pub const DEFAULT_OBJECT: bool = true;

    pub fn array(&self) -> bool {
        self.array.unwrap_or(Self::DEFAULT_ARRAY)
    }

    pub fn object(&self) -> bool {
        self.object.unwrap_or(Self::DEFAULT_OBJECT)
    }
}
