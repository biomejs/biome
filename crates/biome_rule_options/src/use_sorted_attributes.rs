use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};
pub use crate::shared::sort_mode::SortMode;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseSortedAttributesOptions {
  pub sort_mode: SortMode,
}
