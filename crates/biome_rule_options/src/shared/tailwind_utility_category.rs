use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum TailwindUtilityCategory {
    /// Sizing, positioning, margins, and other utilities that don't set appearance.
    Layout,
    Color,
    Typography,
    Spacing,
    Shape,
    Effects,
    Motion,
}
