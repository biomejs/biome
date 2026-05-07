use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

/// Options for the `noTailwindArbitraryValue` rule.
///
/// Controls which attributes and utility functions are checked for arbitrary values.
#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTailwindArbitraryValueOptions {
    /// Additional attributes that will be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Names of the functions or tagged templates that will be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub functions: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoTailwindArbitraryValueOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(attributes) = other.attributes {
            self.attributes = Some(attributes);
        }
        if let Some(functions) = other.functions {
            self.functions = Some(functions);
        }
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions"];

impl Deserializable for NoTailwindArbitraryValueOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, NoTailwindArbitraryValueOptionsVisitor, name)
    }
}

struct NoTailwindArbitraryValueOptionsVisitor;

impl DeserializationVisitor for NoTailwindArbitraryValueOptionsVisitor {
    type Output = NoTailwindArbitraryValueOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        let mut result = NoTailwindArbitraryValueOptions::default();
        let mut attributes = Vec::new();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attrs) = Deserializable::deserialize(ctx, &value, &key_text) {
                        attributes.extend::<Vec<Box<str>>>(attrs);
                    }
                }
                "functions" => {
                    result.functions = Deserializable::deserialize(ctx, &value, &key_text)
                }
                unknown_key => ctx.report(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }
        result.attributes = if attributes.is_empty() {
            None
        } else {
            Some(attributes.into_boxed_slice())
        };
        Some(result)
    }
}
