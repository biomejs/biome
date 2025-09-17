use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum UseConsistentArrowReturnStyle {
    //enforces braces around the function body
    #[default]
    AsNeeded,
    //enforces no braces where they can be omitted (default)
    Always,
    //enforces no braces around the function body (constrains arrow functions to the role of returning an expression)
    Never,
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct UseConsistentArrowReturnOptions {
    pub style: UseConsistentArrowReturnStyle,
    // This option is only applicable when used in conjunction with the "asNeeded" option.
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
                    "`requireForObjectLiteral` can only be used when `style` is `asNeeded`.",
                )
                .with_range(range),
            );
            return false;
        }
        true
    }
}
