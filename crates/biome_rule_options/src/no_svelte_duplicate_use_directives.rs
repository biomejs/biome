use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
/// Options for the `noSvelteDuplicateUseDirectives` rule.
///
/// This rule currently has no configurable options.
pub struct NoSvelteDuplicateUseDirectivesOptions {}
