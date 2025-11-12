use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoJsxLiteralsOptions {
    /// When enabled, also flag string literals inside JSX expressions and attributes
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub no_strings: Option<bool>,

    /// An array of strings that won't trigger the rule. Whitespaces are taken into consideration
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allowed_strings: Option<Box<[Box<str>]>>,

    /// When enabled, strings inside props are always ignored
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_props: Option<bool>,
}

impl biome_deserialize::Merge for NoJsxLiteralsOptions {
    fn merge_with(&mut self, other: Self) {
        self.no_strings.merge_with(other.no_strings);
        self.ignore_props.merge_with(other.ignore_props);
        if let Some(allowed_strings) = other.allowed_strings {
            self.allowed_strings = Some(allowed_strings);
        }
    }
}

impl NoJsxLiteralsOptions {
    pub const DEFAULT_NO_STRINGS: bool = false;
    pub const DEFAULT_IGNORE_PROPS: bool = false;

    /// Returns [`Self::no_strings`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_NO_STRINGS`].
    pub fn no_strings(&self) -> bool {
        self.no_strings.unwrap_or(Self::DEFAULT_NO_STRINGS)
    }

    /// Returns [`Self::ignore_props`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_PROPS`].
    pub fn ignore_props(&self) -> bool {
        self.ignore_props.unwrap_or(Self::DEFAULT_IGNORE_PROPS)
    }
}
