use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};
use biome_rowan::TextRange;

use super::presets::{get_utilities_preset, UseSortedClassesPreset};

const CLASS_ATTRIBUTES: [&str; 2] = ["class", "className"];
const ALLOWED_PRESETS: &[&str] = &["no-preset", "tailwind-css"];

pub struct UtilityLayer {
    pub layer: String,
    pub classes: Vec<String>,
}

impl Deserializable for UtilityLayer {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(UtilityLayerVisitor, name, diagnostics)
    }
}

struct UtilityLayerVisitor;
impl DeserializationVisitor for UtilityLayerVisitor {
    type Output = UtilityLayer;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut layer: Option<String> = None;
        let mut classes: Option<Vec<String>> = None;
        const ALLOWED_OPTIONS: &[&str] = &["layer", "classes"];

        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "layer" => {
                    if let Some(layer_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        layer = Some(layer_option);
                    }
                }
                "classes" => {
                    if let Some(classes_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        classes = Some(classes_option);
                    }
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }

        let missing_layer = layer.is_none();
        let missing_classes = classes.is_none();

        if missing_layer || missing_classes {
            let mut missing_keys: Vec<&str> = Vec::new();
            if missing_layer {
                missing_keys.push("layer");
            }
            if missing_classes {
                missing_keys.push("classes");
            }
            let missing_keys = missing_keys.join(", ");
            // TODO: how to actually handle this?
            diagnostics.push(DeserializationDiagnostic::new(format!(
                "Missing {}.",
                missing_keys
            )));

            None
        } else {
            Some(UtilityLayer {
                layer: layer.expect("TODO: error message (this should never happen)"),
                classes: classes.expect("TODO: error message (this should never happen)"),
            })
        }
    }
}

#[derive(Debug, Clone)]
pub struct UseSortedClassesOptions {
    pub attributes: Vec<String>,
    pub functions: Vec<String>,
    pub utilities: Vec<String>,
}

impl Default for UseSortedClassesOptions {
    fn default() -> Self {
        UseSortedClassesOptions {
            attributes: CLASS_ATTRIBUTES.iter().map(|&s| s.to_string()).collect(),
            functions: Vec::new(),
            utilities: Vec::new(),
        }
    }
}

const ALLOWED_OPTIONS: &[&str] = &["attributes", "functions", "preset", "utilities"];

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
        let mut preset: UseSortedClassesPreset = UseSortedClassesPreset::TailwindCSS;
        let mut utilities_option: Option<Vec<UtilityLayer>> = None;

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
                "preset" => {
                    if let Some(preset_option) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        let preset_option: String = preset_option; // TODO: is there a better way to do this?
                        let preset_option = preset_option.as_str();
                        match preset_option {
                            "tailwind-css" => {
                                preset = UseSortedClassesPreset::TailwindCSS;
                            }
                            "no-preset" => {
                                preset = UseSortedClassesPreset::None;
                            }
                            _ => {
                                diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                                    preset_option,
                                    value.range(),
                                    ALLOWED_PRESETS,
                                ));
                            }
                        }
                    }
                }
                "utilities" => {
                    if let Some(utilities_opt) =
                        Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        utilities_option = Some(utilities_opt);
                    }
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_OPTIONS,
                )),
            }
        }

        let resolved_utilities = match utilities_option {
            Some(utilities) => utilities,
            None => get_utilities_preset(&preset),
        };
        result.utilities = resolved_utilities
            .iter()
            .flat_map(|layer| {
                // TODO: extend layer here
                layer.classes.clone()
            })
            .collect();

        Some(result)
    }
}
