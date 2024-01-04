use crate::configuration::overrides::{
    OverrideFormatterConfiguration, OverrideLinterConfiguration,
    OverrideOrganizeImportsConfiguration, OverridePattern, Overrides,
};
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor, Text,
    VisitableType,
};

impl Deserializable for Overrides {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        Some(Overrides(Deserializable::deserialize(
            value,
            name,
            diagnostics,
        )?))
    }
}

impl Deserializable for OverridePattern {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverridePatternVisitor, name, diagnostics)
    }
}

struct OverridePatternVisitor;
impl DeserializationVisitor for OverridePatternVisitor {
    type Output = OverridePattern;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
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
            "css",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "ignore" => {
                    result.ignore = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "include" => {
                    result.include = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "formatter" => {
                    result.formatter = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "linter" => {
                    result.linter = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "organizeImports" => {
                    result.organize_imports =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
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

impl Deserializable for OverrideFormatterConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverrideFormatterConfigurationVisitor, name, diagnostics)
    }
}

struct OverrideFormatterConfigurationVisitor;
impl DeserializationVisitor for OverrideFormatterConfigurationVisitor {
    type Output = OverrideFormatterConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &[
            "enabled",
            "formatWithErrors",
            "indentStyle",
            "indentSize",
            "indentWidth",
            "lineEnding",
            "lineWidth",
        ];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "enabled" => {
                    result.enabled = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "indentStyle" => {
                    result.indent_style =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "indentSize" => {
                    result.indent_width =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                    diagnostics.push(DeserializationDiagnostic::new_deprecated_use_instead(
                        &key_text,
                        key.range(),
                        "formatter.indentWidth",
                    ));
                }
                "indentWidth" => {
                    result.indent_width =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "lineEnding" => {
                    result.line_ending =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "lineWidth" => {
                    result.line_width = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "formatWithErrors" => {
                    result.format_with_errors =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
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

impl Deserializable for OverrideLinterConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(OverrideLinterConfigurationVisitor, name, diagnostics)
    }
}

struct OverrideLinterConfigurationVisitor;
impl DeserializationVisitor for OverrideLinterConfigurationVisitor {
    type Output = OverrideLinterConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled", "rules"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "enabled" => {
                    result.enabled = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "rules" => {
                    result.rules = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    key_text.text(),
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}

impl Deserializable for OverrideOrganizeImportsConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(
            OverrideOrganizeImportsConfigurationVisitor,
            name,
            diagnostics,
        )
    }
}

struct OverrideOrganizeImportsConfigurationVisitor;
impl DeserializationVisitor for OverrideOrganizeImportsConfigurationVisitor {
    type Output = OverrideOrganizeImportsConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["enabled"];
        let mut result = Self::Output::default();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "enabled" => {
                    result.enabled = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                _ => diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                    key_text.text(),
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(result)
    }
}
