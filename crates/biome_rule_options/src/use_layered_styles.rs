use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseLayeredStylesOptions {
    /// Require `@import` rules to have a cascade layer. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub require_import_layers: Option<bool>,
}

impl UseLayeredStylesOptions {
    pub const DEFAULT_REQUIRE_IMPORT_LAYERS: bool = true;

    /// Returns [`Self::require_import_layers`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_REQUIRE_IMPORT_LAYERS`].
    pub fn require_import_layers(&self) -> bool {
        self.require_import_layers
            .unwrap_or(Self::DEFAULT_REQUIRE_IMPORT_LAYERS)
    }
}
