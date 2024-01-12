use crate::configuration::vcs::{VcsClientKind, VcsConfiguration};
use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableValue, DeserializationDiagnostic, DeserializationVisitor,
    NoneState, Text, VisitableType,
};

impl Deserializable for VcsConfiguration {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        value.deserialize(VcsConfigurationVisitor, name, diagnostics)
    }
}

struct VcsConfigurationVisitor;
impl DeserializationVisitor for VcsConfigurationVisitor {
    type Output = VcsConfiguration;

    const EXPECTED_TYPE: VisitableType = VisitableType::MAP;

    fn visit_map(
        self,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        range: biome_rowan::TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["clientKind", "enabled", "useIgnoreFile", "root"];
        let mut result = Self::Output::none();
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(&key, "", diagnostics) else {
                continue;
            };
            match key_text.text() {
                "clientKind" => {
                    result.client_kind =
                        Some(Deserializable::deserialize(&value, &key_text, diagnostics)?);
                }
                "enabled" => {
                    result.enabled = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "useIgnoreFile" => {
                    result.use_ignore_file =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "root" => {
                    result.root = Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                "defaultBranch" => {
                    result.default_branch =
                        Deserializable::deserialize(&value, &key_text, diagnostics);
                }
                _ => {
                    diagnostics.push(DeserializationDiagnostic::new_unknown_key(
                        &key_text,
                        key.range(),
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
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        let value_text = Text::deserialize(value, name, diagnostics)?;
        if let Ok(value) = value_text.text().parse::<Self>() {
            Some(value)
        } else {
            const ALLOWED_VARIANTS: &[&str] = &["git"];
            diagnostics.push(DeserializationDiagnostic::new_unknown_value(
                value_text.text(),
                value.range(),
                ALLOWED_VARIANTS,
            ));
            None
        }
    }
}
