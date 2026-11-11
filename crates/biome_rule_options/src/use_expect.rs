use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseExpectOptions {
    /// Additional function names that count as assertions, such as
    /// `expectSaga` or `utils.assertValid`.
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub assert_function_names: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for UseExpectOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(assert_function_names) = other.assert_function_names {
            self.assert_function_names = Some(assert_function_names);
        }
    }
}
