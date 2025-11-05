use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveLinesPerFunctionOptions {
    /// The maximum number of lines allowed in a function body.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_lines: Option<NonZeroU16>,
    /// When this options is set to `true`, blank lines in the function body are not counted towards the maximum line limit.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub skip_blank_lines: Option<bool>,
    /// When this option is set to `true`, Immediately Invoked Function Expressions (IIFEs) are not checked for the maximum line limit.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub skip_iifes: Option<bool>,
}

impl NoExcessiveLinesPerFunctionOptions {
    pub const DEFAULT_MAX_LINES: NonZeroU16 = NonZeroU16::new(50).unwrap();
    pub const DEFAULT_SKIP_BLANK_LINES: bool = false;
    pub const DEFAULT_SKIP_IIFES: bool = false;

    /// Returns [`Self::max_lines`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX_LINES`].
    pub fn max_lines(&self) -> NonZeroU16 {
        self.max_lines.unwrap_or(Self::DEFAULT_MAX_LINES)
    }

    /// Returns [`Self::skip_blank_lines`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_SKIP_BLANK_LINES`].
    pub fn skip_blank_lines(&self) -> bool {
        self.skip_blank_lines
            .unwrap_or(Self::DEFAULT_SKIP_BLANK_LINES)
    }

    /// Returns [`Self::skip_iifes`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_SKIP_IIFES`].
    pub fn skip_iifes(&self) -> bool {
        self.skip_iifes.unwrap_or(Self::DEFAULT_SKIP_IIFES)
    }
}
