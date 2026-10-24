use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueLoneTemplateOptions {
    /// Allows a `<template>` without a directive when it has an `id` or `ref`
    /// attribute, written plainly or bound with `v-bind`. Disabled by default.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_accessible: Option<bool>,
}

impl NoVueLoneTemplateOptions {
    pub const DEFAULT_IGNORE_ACCESSIBLE: bool = false;

    /// Returns [`Self::ignore_accessible`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_IGNORE_ACCESSIBLE`].
    pub fn ignore_accessible(&self) -> bool {
        self.ignore_accessible
            .unwrap_or(Self::DEFAULT_IGNORE_ACCESSIBLE)
    }
}
