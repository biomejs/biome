use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

/// Options for the `noExtendNative` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExtendNativeOptions {
    /// Built-in names to ignore. Extending the prototype of an ignored
    /// name will not trigger this rule.
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoExtendNativeOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore);
        }
    }
}
