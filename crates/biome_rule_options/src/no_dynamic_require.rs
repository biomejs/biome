use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDynamicRequireOptions {
    /// Also report `import()` calls whose module name isn't a fixed string.
    /// Disabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub check_dynamic_imports: Option<bool>,
}

impl NoDynamicRequireOptions {
    pub const DEFAULT_CHECK_DYNAMIC_IMPORTS: bool = false;

    /// Returns [`Self::check_dynamic_imports`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CHECK_DYNAMIC_IMPORTS`].
    pub fn check_dynamic_imports(&self) -> bool {
        self.check_dynamic_imports
            .unwrap_or(Self::DEFAULT_CHECK_DYNAMIC_IMPORTS)
    }
}
