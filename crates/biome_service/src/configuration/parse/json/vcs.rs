use crate::configuration::vcs::{VcsClientKind, VcsConfiguration};
use biome_console::markup;
use biome_deserialize::json::{report_unknown_map_key, report_unknown_variant, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, VisitNode};
use biome_json_syntax::{AnyJsonValue, JsonLanguage, JsonStringValue};
use biome_rowan::{AstNode, SyntaxNode};

impl VcsConfiguration {
    const ALLOWED_KEYS: &'static [&'static str] =
        &["clientKind", "enabled", "useIgnoreFile", "root"];
}

impl VisitNode<JsonLanguage> for VcsConfiguration {
    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value)?;
        let name_text = name.inner_string_text().ok()?;
        let name_text = name_text.text();
        match name_text {
            "clientKind" => {
                let mut client_kind = VcsClientKind::default();
                client_kind.map_to_known_string(&value, name_text, diagnostics)?;
                self.client_kind = Some(client_kind);
            }
            "enabled" => {
                self.enabled = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "useIgnoreFile" => {
                self.use_ignore_file = self.map_to_boolean(&value, name_text, diagnostics);
            }
            "root" => {
                self.root = self.map_to_string(&value, name_text, diagnostics);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }
        Some(())
    }
}

impl VcsClientKind {
    const ALLOWED_VARIANTS: &'static [&'static str] = &["git"];
}

impl VisitNode<JsonLanguage> for VcsClientKind {
    fn visit_value(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let node = JsonStringValue::cast_ref(node)?;
        if node.inner_string_text().ok()?.text() == "git" {
            *self = VcsClientKind::Git;
        } else {
            report_unknown_variant(&node, Self::ALLOWED_VARIANTS, diagnostics);
        }
        Some(())
    }
}

pub(crate) fn validate_vcs_configuration(
    node: &AnyJsonValue,
    configuration: &mut VcsConfiguration,
    diagnostics: &mut Vec<DeserializationDiagnostic>,
) {
    if configuration.client_kind.is_none() && !configuration.is_disabled() {
        diagnostics.push(
            DeserializationDiagnostic::new(markup! {
                "You enabled the VCS integration, but you didn't specify a client."
            })
            .with_range(node.range())
            .with_note("Biome will disable the VCS integration until the issue is fixed."),
        );
        configuration.enabled = Some(false);
    }
}
