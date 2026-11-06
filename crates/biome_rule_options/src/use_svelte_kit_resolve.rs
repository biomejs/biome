use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSvelteKitResolveOptions {
    /// Whether to ignore all `goto()` calls. Default: `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_goto: Option<bool>,

    /// Whether to ignore all `<a>` elements. Default: `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_links: Option<bool>,

    /// Whether to ignore all `pushState()` calls. Default: `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_push_state: Option<bool>,

    /// Whether to ignore all `replaceState()` calls. Default: `false`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_replace_state: Option<bool>,
}

impl UseSvelteKitResolveOptions {
    pub const DEFAULT_IGNORE_GOTO: bool = false;
    pub const DEFAULT_IGNORE_LINKS: bool = false;
    pub const DEFAULT_IGNORE_PUSH_STATE: bool = false;
    pub const DEFAULT_IGNORE_REPLACE_STATE: bool = false;

    /// Returns [`Self::ignore_goto`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_GOTO`].
    pub fn ignore_goto(&self) -> bool {
        self.ignore_goto.unwrap_or(Self::DEFAULT_IGNORE_GOTO)
    }

    /// Returns [`Self::ignore_links`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_LINKS`].
    pub fn ignore_links(&self) -> bool {
        self.ignore_links.unwrap_or(Self::DEFAULT_IGNORE_LINKS)
    }

    /// Returns [`Self::ignore_push_state`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_PUSH_STATE`].
    pub fn ignore_push_state(&self) -> bool {
        self.ignore_push_state
            .unwrap_or(Self::DEFAULT_IGNORE_PUSH_STATE)
    }

    /// Returns [`Self::ignore_replace_state`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_REPLACE_STATE`].
    pub fn ignore_replace_state(&self) -> bool {
        self.ignore_replace_state
            .unwrap_or(Self::DEFAULT_IGNORE_REPLACE_STATE)
    }
}
