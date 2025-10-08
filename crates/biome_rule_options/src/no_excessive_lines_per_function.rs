use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveLinesPerFunctionOptions {
    /// The maximum number of lines allowed in a function body.
    pub max_lines: NonZeroU16,
    /// When this options is set to `true`, blank lines in the function body are not counted towards the maximum line limit.
    pub skip_blank_lines: bool,
    /// When this option is set to `true`, Immediately Invoked Function Expressions (IIFEs) are not checked for the maximum line limit.
    pub skip_iifes: bool,
}

impl Default for NoExcessiveLinesPerFunctionOptions {
    fn default() -> Self {
        Self {
            max_lines: NonZeroU16::new(50).unwrap(),
            skip_blank_lines: false,
            skip_iifes: false,
        }
    }
}
