use biome_console::markup;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic, TextRange,
};
use biome_deserialize_macros::Deserializable;
use rustc_hash::FxHashSet;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
#[deserializable(with_validator)]
pub struct UseUniqueElementIdsOptions {
    /// Component names that accept an `id` prop that does not translate
    /// to a DOM element id.
    pub excluded_components: FxHashSet<Box<str>>,
}

impl DeserializableValidator for UseUniqueElementIdsOptions {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: TextRange,
    ) -> bool {
        for name in &self.excluded_components {
            let msg = if name.is_empty() {
                "empty values"
            } else if name.contains('.') {
                "values with dots"
            } else {
                continue;
            };
            ctx.report(
                DeserializationDiagnostic::new(markup!(
                    <Emphasis>"'excludedComponents'"</Emphasis>" does not accept "{msg}"."
                ))
                .with_range(range),
            );
            return false;
        }

        true
    }
}
