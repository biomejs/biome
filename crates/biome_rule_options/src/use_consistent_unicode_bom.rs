use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseConsistentUnicodeBomOptions {
    /// Whether files must start with a Unicode byte order mark. Defaults to `"never"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bom: Option<UnicodeBom>,
}

impl UseConsistentUnicodeBomOptions {
    pub const DEFAULT_BOM: UnicodeBom = UnicodeBom::Never;

    /// Returns [`Self::bom`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_BOM`].
    pub fn bom(&self) -> UnicodeBom {
        self.bom.unwrap_or(Self::DEFAULT_BOM)
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum UnicodeBom {
    /// Require a Unicode byte order mark at the start of every file.
    Always,
    /// Disallow a Unicode byte order mark at the start of every file.
    #[default]
    Never,
}
