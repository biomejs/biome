use crate::configuration::vcs::{VcsClientKind, VcsConfiguration};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    ExpectedType,
};
use biome_rowan::TokenText;

impl Deserializable for VcsConfiguration {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(VcsConfigurationVisitor, diagnostics)
    }
}

struct VcsConfigurationVisitor;
impl DeserializationVisitor for VcsConfigurationVisitor {
    type Output = VcsConfiguration;

    const EXPECTED_TYPE: ExpectedType = ExpectedType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = (impl DeserializableValue, impl DeserializableValue)>,
        range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["clientKind", "enabled", "useIgnoreFile", "root"];
        let mut result = Self::Output::default();
        for (key, value) in members {
            let Some(key) = TokenText::deserialize(key, diagnostics) else {
                continue;
            };
            let key = key.text();
            match key {
                "clientKind" => {
                    result.client_kind = Some(Deserializable::deserialize(value, diagnostics)?);
                }
                "enabled" => {
                    result.enabled = Deserializable::deserialize(value, diagnostics);
                }
                "useIgnoreFile" => {
                    result.use_ignore_file = Deserializable::deserialize(value, diagnostics);
                }
                "root" => {
                    result.root = Deserializable::deserialize(value, diagnostics);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        key,
                        range,
                        ALLOWED_KEYS,
                    ));
                }
            }
        }
        if result.client_kind.is_none() && !result.is_disabled() {
            diagnostics.push(
                DeserializationDiagnostic::new(markup! {
                    "You enabled the VCS integration, but you didn't specify a client."
                })
                .with_range(range)
                .with_note("Biome will disable the VCS integration until the issue is fixed."),
            );
            result.enabled = Some(false);
        }
        Some(result)
    }
}

impl Deserializable for VcsClientKind {
    fn deserialize(
        value: impl DeserializableValue,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        const ALLOWED_VARIANTS: &[&str] = &["git"];
        let range = value.range();
        let value = TokenText::deserialize(value, diagnostics)?;
        if let Ok(value) = value.text().parse::<Self>() {
            Some(value)
        } else {
            diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                value.text(),
                range,
                ALLOWED_VARIANTS,
            ));
            None
        }
    }
}
