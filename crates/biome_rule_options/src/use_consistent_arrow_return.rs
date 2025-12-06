use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

/// Options for the `useConsistentArrowReturn` rule.
#[derive(Default, Clone, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct UseConsistentArrowReturnOptions {
    /// The style to enforce for arrow function return statements.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub style: Option<UseConsistentArrowReturnStyle>,
    /// Determines whether the rule enforces a consistent style when the return value is an object literal.
    ///
    /// This option is only applicable when used in conjunction with the `asNeeded` option.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub require_for_object_literal: Option<bool>,
}

impl DeserializableValidator for UseConsistentArrowReturnOptions {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self.require_for_object_literal.unwrap_or_default()
            && self.style.unwrap_or_default() != UseConsistentArrowReturnStyle::AsNeeded
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

#[derive(
    Clone, Copy, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize, Default,
)]
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
