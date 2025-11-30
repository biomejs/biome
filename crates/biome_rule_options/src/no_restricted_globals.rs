use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoRestrictedGlobalsOptions {
    /// A list of names that should trigger the rule
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub denied_globals: Option<FxHashMap<Box<str>, Box<str>>>,
}

impl biome_deserialize::Merge for NoRestrictedGlobalsOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(denied_globals) = other.denied_globals {
            self.denied_globals = Some(denied_globals);
        }
    }
}
