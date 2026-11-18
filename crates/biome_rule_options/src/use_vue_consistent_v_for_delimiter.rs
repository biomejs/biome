use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentVForDelimiterOptions {
    /// The word to use between the item and the list in `v-for` directives:
    /// `"in"` or `"of"`. Defaults to `"in"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub delimiter: Option<VueVForDelimiter>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VueVForDelimiter {
    #[default]
    In,
    Of,
}

impl biome_console::fmt::Display for VueVForDelimiter {
    fn fmt(&self, fmt: &mut biome_console::fmt::Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str(match self {
            Self::In => "in",
            Self::Of => "of",
        })
    }
}

impl UseVueConsistentVForDelimiterOptions {
    pub const DEFAULT_DELIMITER: VueVForDelimiter = VueVForDelimiter::In;

    /// Returns [`Self::delimiter`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_DELIMITER`].
    pub fn delimiter(&self) -> VueVForDelimiter {
        self.delimiter.unwrap_or(Self::DEFAULT_DELIMITER)
    }
}
