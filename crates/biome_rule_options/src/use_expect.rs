use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseExpectOptions {
    /// Additional function names that count as assertions, such as
    /// `expectSaga` or `utils.assertValid`.
    #[serde(default, skip_serializing_if = "Option::<_>::is_none")]
    pub assert_function_names: Option<Box<[Box<str>]>>,
}
