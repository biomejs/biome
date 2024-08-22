use crate::{LanguageRoot, Manifest};
use biome_deserialize::json::deserialize_from_json_ast;
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationDiagnostic,
    DeserializationVisitor, Deserialized, Text,
};
use biome_json_syntax::JsonLanguage;
use biome_text_size::TextRange;
use rustc_hash::FxHashMap;

#[derive(Debug, Default, Clone)]
pub struct TsConfigJson {
    base_url: Option<String>,
    paths: FxHashMap<String, Vec<String>>,
}

impl Manifest for TsConfigJson {
    type Language = JsonLanguage;

    fn deserialize_manifest(root: &LanguageRoot<Self::Language>) -> Deserialized<Self> {
        deserialize_from_json_ast::<TsConfigJson>(root, "")
    }
}

impl Deserializable for TsConfigJson {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(TsConfigJsonVisitor, name, diagnostics)
    }
}

struct TsConfigJsonVisitor;
impl DeserializationVisitor for TsConfigJsonVisitor {
    type Output = TsConfigJson;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "baseUrl" => {
                    result.base_url = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "paths" => {
                    if let Some(paths) = Deserializable::deserialize(&value, &key_text, diagnostics)
                    {
                        result.paths = paths;
                    }
                }
                _ => {
                    // For now, we ignore unknown keys, because we might not need to support all of them
                }
            }
        }
        Some(result)
    }
}
