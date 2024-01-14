use crate::configuration::vcs::VcsConfiguration;
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
        const ALLOWED_KEYS: &[&str] = &[
            "clientKind",
            "defaultBranch",
            "enabled",
            "root",
            "useIgnoreFile",
        ];
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
                DeserializationDiagnostic::new(
                    "You enabled the VCS integration, but you didn't specify a client.",
                )
                .with_range(range)
                .with_note("Biome will disable the VCS integration until the issue is fixed."),
            );
            result.enabled = Some(false);
        }
        Some(result)
    }
}
