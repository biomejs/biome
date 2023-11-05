use crate::configuration::organize_imports::OrganizeImports;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::{TextRange, TokenText};

impl Deserializable for OrganizeImports {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OrganizeImportsVisitor, diagnostics)
    }
}

struct OrganizeImportsVisitor;
impl DeserializationVisitor for OrganizeImportsVisitor {
    type Output = OrganizeImports;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled", "ignore", "include"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "enabled" => {
                    result.enabled = Deserializable::deserialize(value, diagnostics);
                }
                "ignore" => {
                    result.ignore = Deserializable::deserialize(value, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(value, diagnostics);
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
