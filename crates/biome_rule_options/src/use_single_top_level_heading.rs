//! Configuration for the heading level checked by `useSingleTopLevelHeading`.
//!
//! The level must be between 1 and 6 and defaults to 1.

use std::num::NonZeroU8;

use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct UseSingleTopLevelHeadingOptions {
    /// The heading level that is treated as the document's top-level heading.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub level: Option<NonZeroU8>,
}

impl UseSingleTopLevelHeadingOptions {
    /// The heading level used when no level is configured.
    pub const DEFAULT_LEVEL: NonZeroU8 = NonZeroU8::new(1).unwrap();

    /// Returns the configured heading level, or the default level, as a `u8`.
    pub fn level(&self) -> u8 {
        self.level.unwrap_or(Self::DEFAULT_LEVEL).get()
    }
}

impl DeserializableValidator for UseSingleTopLevelHeadingOptions {
    fn validate(
        &mut self,
        ctx: &mut dyn DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self.level.is_some_and(|level| level.get() > 6) {
            ctx.report(
                DeserializationDiagnostic::new("The heading level must be between 1 and 6.")
                    .with_range(range),
            );
            return false;
        }

        true
    }
}
