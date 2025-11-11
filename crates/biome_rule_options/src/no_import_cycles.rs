use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoImportCyclesOptions {
    /// Ignores type-only imports when finding an import cycle. A type-only import (`import type`)
    /// will be removed by the compiler, so it cuts an import cycle at runtime. Note that named type
    /// imports (`import { type Foo }`) aren't considered as type-only because it's not removed by
    /// the compiler if the `verbatimModuleSyntax` option is enabled. Enabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_types: Option<bool>,
}

impl NoImportCyclesOptions {
    pub const DEFAULT_IGNORE_TYPES: bool = true;

    /// Returns [`Self::ignore_types`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_TYPES`].
    pub fn ignore_types(&self) -> bool {
        self.ignore_types.unwrap_or(Self::DEFAULT_IGNORE_TYPES)
    }
}
