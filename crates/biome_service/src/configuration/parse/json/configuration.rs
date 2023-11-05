use crate::Configuration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::TokenText;

impl Deserializable for Configuration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(ConfigurationVisitor, diagnostics)
    }
}

struct ConfigurationVisitor;
impl DeserializationVisitor for ConfigurationVisitor {
    type Output = Configuration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "vcs",
            "files",
            "linter",
            "formatter",
            "javascript",
            "json",
            "$schema",
            "organizeImports",
            "extends",
            "overrides",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "$schema" => {
                    result.schema = Deserializable::deserialize(value, diagnostics);
                }
                "files" => {
                    result.files = Deserializable::deserialize(value, diagnostics);
                }
                "vcs" => {
                    result.vcs = Deserializable::deserialize(value, diagnostics);
                }
                "formatter" => {
                    result.formatter = Deserializable::deserialize(value, diagnostics);
                }
                "linter" => {
                    result.linter = Deserializable::deserialize(value, diagnostics);
                }
                "javascript" => {
                    result.javascript = Deserializable::deserialize(value, diagnostics);
                }
                "json" => {
                    result.json = Deserializable::deserialize(value, diagnostics);
                }
                "organizeImports" => {
                    result.organize_imports = Deserializable::deserialize(value, diagnostics);
                }
                "extends" => {
                    result.extends = Deserializable::deserialize(value, diagnostics);
                }
                "overrides" => {
                    result.overrides = Deserializable::deserialize(value, diagnostics);
                }
                _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    key.text(),
                    key_range,
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}
