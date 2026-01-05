use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseInputNameOptions {
    /// Check that the input type name follows the convention <mutationName>Input
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_input_type: Option<bool>,

    /// Treat input type names as case-sensitive
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub case_sensitive_input_type: Option<bool>,
}

impl UseInputNameOptions {
    pub const DEFAULT_CHECK_INPUT_TYPE: bool = false;
    pub const DEFAULT_CASE_SENSITIVE_INPUT_TYPE: bool = true;

    pub fn check_input_type(&self) -> bool {
        self.check_input_type
            .unwrap_or(Self::DEFAULT_CHECK_INPUT_TYPE)
    }

    pub fn case_sensitive_input_type(&self) -> bool {
        self.case_sensitive_input_type
            .unwrap_or(Self::DEFAULT_CASE_SENSITIVE_INPUT_TYPE)
    }
}
