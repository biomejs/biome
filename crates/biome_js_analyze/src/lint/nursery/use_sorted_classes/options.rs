use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationDiagnostic,
    DeserializationVisitor, Text,
};
use biome_rowan::TextRange;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Attributes that are always targets.
const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

#[derive(Default, Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UtilityClassSortingOptions {
    /// Additional attributes that will be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Box<[Box<str>]>>,
    /// Names of the functions or tagged templates that will be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<Box<str>>>,
}

impl UtilityClassSortingOptions {
    pub(crate) fn has_function(&self, name: &str) -> bool {
        let iter = self.functions.iter().flatten();
        for v in iter {
            if v.as_ref() == name {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_attribute(&self, name: &str) -> bool {
        CLASS_ATTRIBUTES.contains(&name)
            || self.attributes.iter().flatten().any(|v| v.as_ref() == name)
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions"];

impl Deserializable for UtilityClassSortingOptions {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(UtilityClassSortingOptionsVisitor, name, diagnostics)
    }
}

struct UtilityClassSortingOptionsVisitor;
impl DeserializationVisitor for UtilityClassSortingOptionsVisitor {
    type Output = UtilityClassSortingOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = UtilityClassSortingOptions::default();

        let mut attributes = Vec::new();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        attributes.extend::<Vec<Box<str>>>(attributes_option);
                    }
                }
                "functions" => {
                    result.functions = Deserializable::deserialize(&value, &key_text, diagnostics)
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
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
