use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConvexAccessControlOptions {
    /// Names of functions that count as access control functions, in addition to the names
    /// matched by `prefixes`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<Box<[Box<str>]>>,

    /// Words that start the name of an access control function. A name matches a prefix when it's
    /// the prefix on its own, or the prefix followed by an uppercase letter or `_`.
    ///
    /// Setting this option replaces the default prefixes: `assert`, `can`, `check`, `ensure`,
    /// `has` and `require`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub prefixes: Option<Box<[Box<str>]>>,
}

impl UseConvexAccessControlOptions {
    /// Sorted prefixes used when [`Self::prefixes`] isn't set.
    pub const DEFAULT_PREFIXES: &'static [&'static str] =
        &["assert", "can", "check", "ensure", "has", "require"];

    /// Returns [`Self::functions`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn functions(&self) -> &[Box<str>] {
        self.functions.as_deref().unwrap_or_default()
    }
}

impl biome_deserialize::Merge for UseConvexAccessControlOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(functions) = other.functions {
            self.functions = Some(functions);
        }
        if let Some(prefixes) = other.prefixes {
            self.prefixes = Some(prefixes);
        }
    }
}
