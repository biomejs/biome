use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Merge, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoReactNativeRawTextOptions {
    /// Names of additional components that are allowed to contain raw text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip: Option<Vec<String>>,
}
