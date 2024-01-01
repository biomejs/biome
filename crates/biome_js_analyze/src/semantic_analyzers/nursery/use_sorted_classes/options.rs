use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_rowan::TextRange;

/// Attributes that are always targets.
const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];

#[derive(Debug, Clone)]
pub struct UseSortedClassesOptions {
    /// Additional attribute targets specified by the user.
    pub attributes: Vec<String>,
    /// Function targets specified by the user.
    pub functions: Vec<String>,
}

impl Default for UseSortedClassesOptions {
    fn default() -> Self {
        UseSortedClassesOptions {
            attributes: CLASS_ATTRIBUTES.iter().map(|&s| s.to_string()).collect(),
            functions: Vec::new(),
        }
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions"];

impl Deserializable for UseSortedClassesOptions {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(UseSortedClassesOptionsVisitor, name, diagnostics)
    }
}

struct UseSortedClassesOptionsVisitor;
impl DeserializationVisitor for UseSortedClassesOptionsVisitor {
    type Output = UseSortedClassesOptions;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = UseSortedClassesOptions::default();
        result
            .attributes
            .extend(CLASS_ATTRIBUTES.iter().map(|&s| s.to_string()));

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "attributes" => {
                    if let Some(attributes_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        let attributes_option: Vec<String> = attributes_option; // TODO: is there a better way to do this?
                        result.attributes.extend(attributes_option);
                    }
                }
                "functions" => {
                    if let Some(functions) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        result.functions = functions;
                    }
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
