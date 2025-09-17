use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseTanStackQueryExhaustiveDepsOptions {
    /// Enable/disable checking useQuery hooks. Defaults to true.
    pub use_query: bool,
    /// Enable/disable checking useInfiniteQuery hooks. Defaults to true.
    pub use_infinite_query: bool,
}

impl Default for UseTanStackQueryExhaustiveDepsOptions {
    fn default() -> Self {
        Self {
            use_query: true,
            use_infinite_query: true,
        }
    }
}
