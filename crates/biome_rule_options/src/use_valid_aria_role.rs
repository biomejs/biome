use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseValidAriaRoleOptions {
    /// It allows specifying a list of roles that might be invalid otherwise
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow_invalid_roles: Option<Box<[Box<str>]>>,
    /// Use this option to ignore non-DOM elements, such as custom components
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_non_dom: Option<bool>,
}

impl biome_deserialize::Merge for UseValidAriaRoleOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allow_invalid_roles) = other.allow_invalid_roles {
            self.allow_invalid_roles = Some(allow_invalid_roles);
        }
        self.ignore_non_dom.merge_with(other.ignore_non_dom);
    }
}

impl UseValidAriaRoleOptions {
    pub const DEFAULT_IGNORE_NON_DOM: bool = false;

    /// Returns [`Self::ignore_non_dom`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_NON_DOM`].
    pub fn ignore_non_dom(&self) -> bool {
        self.ignore_non_dom.unwrap_or(Self::DEFAULT_IGNORE_NON_DOM)
    }
}
