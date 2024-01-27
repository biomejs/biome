use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_rowan::TextRange;
#[cfg(feature = "schemars")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Attributes that are always targets.
const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

#[derive(Deserialize, Serialize, Eq, PartialEq, Debug, Clone)]
#[cfg_attr(feature = "schemars", derive(JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct UtilityClassSortingOptions {
    /// Additional attributes that will be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    /// Names of the functions or tagged templates that will be sorted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<String>>,
}

impl Default for UtilityClassSortingOptions {
    fn default() -> Self {
        UtilityClassSortingOptions {
            attributes: Some(CLASS_ATTRIBUTES.iter().map(|&s| s.to_string()).collect()),
            functions: None,
        }
    }
}

impl UtilityClassSortingOptions {
    pub(crate) fn has_function(&self, name: &str) -> bool {
        let iter = self.functions.iter().flatten();
        for v in iter {
            if v.as_str() == name {
                return true;
            }
        }
        false
    }

    pub(crate) fn has_attribute(&self, name: &str) -> bool {
        let iter = self.attributes.iter().flatten();
        for v in iter {
            if v.as_str() == name {
                return true;
            }
        }
        false
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

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = UtilityClassSortingOptions::default();

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        result
                            .attributes
                            .get_or_insert_with(Vec::new)
                            .extend::<Vec<String>>(attributes_option);
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

        Some(result)
    }
}
