use std::num::NonZeroU16;

use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoExcessiveClassesPerFileOptions {
    /// The maximum number of classes allowed in a file.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub max_classes: Option<NonZeroU16>,
}

impl NoExcessiveClassesPerFileOptions {
    pub const DEFAULT_MAX_CLASSES: NonZeroU16 = NonZeroU16::new(1).unwrap();

    /// Returns [`Self::max_classes`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_MAX_CLASSES`].
    pub fn max_classes(&self) -> NonZeroU16 {
        self.max_classes.unwrap_or(Self::DEFAULT_MAX_CLASSES)
    }
}
