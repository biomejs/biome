use crate::configuration::FilesConfiguration;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::{TextRange, TokenText};

impl Deserializable for FilesConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(FilesConfigurationVisitor, diagnostics)
    }
}

struct FilesConfigurationVisitor;
impl DeserializationVisitor for FilesConfigurationVisitor {
    type Output = FilesConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["maxSize", "ignore", "include", "ignoreUnknown"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "maxSize" => {
                    result.max_size = Deserializable::deserialize(value, diagnostics);
                }
                "ignore" => {
                    result.ignore = Deserializable::deserialize(value, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(value, diagnostics);
                }
                "ignoreUnknown" => {
                    result.ignore_unknown = Deserializable::deserialize(value, diagnostics);
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
