use crate::configuration::overrides::{
    OverrideFormatterConfiguration, OverrideLinterConfiguration,
    OverrideOrganizeImportsConfiguration, OverridePattern, Overrides,
};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::TokenText;

impl Deserializable for Overrides {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        Some(Overrides(Deserializable::deserialize(value, diagnostics)?))
    }
}

impl Deserializable for OverridePattern {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverridePatternVisitor, diagnostics)
    }
}

struct OverridePatternVisitor;
impl DeserializationVisitor for OverridePatternVisitor {
    type Output = OverridePattern;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "ignore",
            "include",
            "formatter",
            "linter",
            "organizeImports",
            "javascript",
            "json",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let key_range = key.range();
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            match key.text() {
                "ignore" => {
                    result.ignore = Deserializable::deserialize(value, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(value, diagnostics);
                }
                "formatter" => {
                    result.formatter = Deserializable::deserialize(value, diagnostics);
                }
                "linter" => {
                    result.linter = Deserializable::deserialize(value, diagnostics);
                }
                "organizeImports" => {
                    result.organize_imports = Deserializable::deserialize(value, diagnostics);
                }
                "javascript" => {
                    result.javascript = Deserializable::deserialize(value, diagnostics);
                }
                "json" => {
                    result.json = Deserializable::deserialize(value, diagnostics);
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

impl Deserializable for OverrideFormatterConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverrideFormatterConfigurationVisitor, diagnostics)
    }
}

struct OverrideFormatterConfigurationVisitor;
impl DeserializationVisitor for OverrideFormatterConfigurationVisitor {
    type Output = OverrideFormatterConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "enabled",
            "formatWithErrors",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineWidth",
        ];
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
                "indentStyle" => {
                    result.indent_style = Deserializable::deserialize(value, diagnostics);
                }
                "indentSize" => {
                    result.indent_width = Deserializable::deserialize(value, diagnostics);
                    diagnostics.push(DeserializationDiagnostic::new_deprecated(
                        key.text(),
                        key_range,
                        "formatter.indentWidth",
                    ));
                }
                "indentWidth" => {
                    result.indent_width = Deserializable::deserialize(value, diagnostics);
                }
                "lineWidth" => {
                    result.line_width = Deserializable::deserialize(value, diagnostics);
                }
                "formatWithErrors" => {
                    result.format_with_errors = Deserializable::deserialize(value, diagnostics);
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

impl Deserializable for OverrideLinterConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverrideLinterConfigurationVisitor, diagnostics)
    }
}

struct OverrideLinterConfigurationVisitor;
impl DeserializationVisitor for OverrideLinterConfigurationVisitor {
    type Output = OverrideLinterConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled", "rules"];
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
                "rules" => {
                    result.rules = Deserializable::deserialize(value, diagnostics);
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

impl Deserializable for OverrideOrganizeImportsConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverrideOrganizeImportsConfigurationVisitor, diagnostics)
    }
}

struct OverrideOrganizeImportsConfigurationVisitor;
impl DeserializationVisitor for OverrideOrganizeImportsConfigurationVisitor {
    type Output = OverrideOrganizeImportsConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        _range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled"];
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
