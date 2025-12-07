use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSelfClosingElementsOptions {
    // Whether or not to ignore checking native HTML elements. Default is false.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_html_elements: Option<bool>,
}

impl UseSelfClosingElementsOptions {
    pub const DEFAULT_IGNORE_HTML_ELEMENTS: bool = false;

    /// Returns [`Self::ignore_html_elements`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_HTML_ELEMENTS`].
    pub fn ignore_html_elements(&self) -> bool {
        self.ignore_html_elements
            .unwrap_or(Self::DEFAULT_IGNORE_HTML_ELEMENTS)
    }
}
