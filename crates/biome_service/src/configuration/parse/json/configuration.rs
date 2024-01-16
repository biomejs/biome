use crate::Configuration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    NoneState, Text, VisitableType,
};

impl Deserializable for Configuration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(ConfigurationVisitor, name, diagnostics)
    }
}

struct ConfigurationVisitor;
impl DeserializationVisitor for ConfigurationVisitor {
    type Output = Configuration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "vcs",
            "files",
            "linter",
            "formatter",
            "javascript",
            "json",
            "css",
            "$schema",
            "organizeImports",
            "extends",
            "overrides",
        ];
        let mut result = Self::Output::none();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "$schema" => {
                    result.schema = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "files" => {
                    result.files = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "vcs" => {
                    result.vcs = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "formatter" => {
                    result.formatter = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "linter" => {
                    result.linter = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "javascript" => {
                    result.javascript = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "json" => {
                    result.json = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "css" => {
                    result.css = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "organizeImports" => {
                    result.organize_imports =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "extends" => {
                    result.extends = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "overrides" => {
                    result.overrides = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                unknown_key => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    unknown_key,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}
