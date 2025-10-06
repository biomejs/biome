use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum UseConsistentArrowReturnStyle {
    /// Enforces no braces where they can be omitted (default).
    #[default]
    AsNeeded,
    /// Enforces braces around the function body.
    Always,
    /// Enforces no braces around the function body (constrains arrow functions to the role of returning an expression).
    Never,
}

/// Options for the `useConsistentArrowReturn` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct UseConsistentArrowReturnOptions {
    /// The style to enforce for arrow function return statements.
    pub style: UseConsistentArrowReturnStyle,
    /// Determines whether the rule enforces a consistent style when the return value is an object literal.
    ///
    /// This option is only applicable when used in conjunction with the `asNeeded` option.
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
