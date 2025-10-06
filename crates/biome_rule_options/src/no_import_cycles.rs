use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoImportCyclesOptions {
    /// Ignores type-only imports when finding an import cycle. A type-only import (`import type`)
    /// will be removed by the compiler, so it cuts an import cycle at runtime. Note that named type
    /// imports (`import { type Foo }`) aren't considered as type-only because it's not removed by
    /// the compiler if the `verbatimModuleSyntax` option is enabled. Enabled by default.
    #[serde(default = "ignore_types_default")]
    pub ignore_types: bool,
}

impl Default for NoImportCyclesOptions {
    fn default() -> Self {
        Self {
            ignore_types: ignore_types_default(),
        }
    }
}

#[inline]
fn ignore_types_default() -> bool {
    true
}
