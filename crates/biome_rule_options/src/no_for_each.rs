use biome_console::markup;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic, TextRange,
};
use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct NoForEachOptions {
    /// A list of variable names allowed for `forEach` calls.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allowed_identifiers: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoForEachOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allowed_identifiers) = other.allowed_identifiers {
            self.allowed_identifiers = Some(allowed_identifiers);
        }
    }
}

impl DeserializableValidator for NoForEachOptions {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        if self
            .allowed_identifiers
            .iter()
            .flatten()
            .any(|identifier| identifier.is_empty() || identifier.contains('.'))
        {
            ctx
                .report(
                    DeserializationDiagnostic::new(markup!(
                        <Emphasis>"'allowedIdentifiers'"</Emphasis>" does not accept empty values or values with dots."
                    ))
                        .with_range(range)
                );
            return false;
        }

        true
    }
}
