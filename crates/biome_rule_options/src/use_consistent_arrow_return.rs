use biome_deserialize::{
    Deserializable, DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum UseConsistentArrowReturnStyle {
    #[default]
    AsNeeded,
    Always,
    Never,
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct UseConsistentArrowReturnOptions {
    pub style: UseConsistentArrowReturnStyle,
    pub require_for_object_literal: bool,
}

impl DeserializableValidator for UseConsistentArrowReturnOptions {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self.require_for_object_literal && self.style != UseConsistentArrowReturnStyle::AsNeeded
        {
            ctx.report(
                DeserializationDiagnostic::new(
                    "`require_for_object_literal` can only be used when `style` is `asNeeded`.",
                )
                .with_range(range),
            );
            return false;
        }
        true
    }
}
