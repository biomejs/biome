use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoDuplicateTypeConstituentsOptions {
    /// Whether to skip checking intersection types (`A & B`).
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_intersections: Option<bool>,
    /// Whether to skip checking union types (`A | B`), including `undefined` in
    /// the type of an optional parameter.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_unions: Option<bool>,
}

impl NoDuplicateTypeConstituentsOptions {
    pub const DEFAULT_IGNORE_INTERSECTIONS: bool = false;
    pub const DEFAULT_IGNORE_UNIONS: bool = false;

    /// Returns [`Self::ignore_intersections`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_INTERSECTIONS`].
    pub fn ignore_intersections(&self) -> bool {
        self.ignore_intersections
            .unwrap_or(Self::DEFAULT_IGNORE_INTERSECTIONS)
    }

    /// Returns [`Self::ignore_unions`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_UNIONS`].
    pub fn ignore_unions(&self) -> bool {
        self.ignore_unions.unwrap_or(Self::DEFAULT_IGNORE_UNIONS)
    }
}
