use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueUselessVBindOptions {
    /// Doesn't report a `v-bind` directive whose value contains a comment, such
    /// as `:id="'foo' /* note */"`. Disabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_comments: Option<bool>,

    /// Doesn't report a `v-bind` directive whose string contains an escape
    /// sequence for a special character, such as `:title="'Line 1\nLine 2'"`.
    /// Disabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_string_escapes: Option<bool>,
}

impl NoVueUselessVBindOptions {
    pub const DEFAULT_IGNORE_COMMENTS: bool = false;
    pub const DEFAULT_IGNORE_STRING_ESCAPES: bool = false;

    /// Returns [`Self::ignore_comments`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_COMMENTS`].
    pub fn ignore_comments(&self) -> bool {
        self.ignore_comments
            .unwrap_or(Self::DEFAULT_IGNORE_COMMENTS)
    }

    /// Returns [`Self::ignore_string_escapes`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_STRING_ESCAPES`].
    pub fn ignore_string_escapes(&self) -> bool {
        self.ignore_string_escapes
            .unwrap_or(Self::DEFAULT_IGNORE_STRING_ESCAPES)
    }
}
