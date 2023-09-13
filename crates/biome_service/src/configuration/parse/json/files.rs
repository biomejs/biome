use crate::configuration::FilesConfiguration;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_json_syntax::JsonLanguage;
use biome_rowan::SyntaxNode;
use std::num::NonZeroU64;

impl VisitNode<JsonLanguage> for FilesConfiguration {
    fn visit_member_name(
        &mut self,
        node: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, FilesConfiguration::KNOWN_KEYS, diagnostics)
    }

    fn visit_map(
        &mut self,
        key: &SyntaxNode<JsonLanguage>,
        value: &SyntaxNode<JsonLanguage>,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        let (name, value) = self.get_key_and_value(key, value, diagnostics)?;
        let name_text = name.text();
        match name_text {
            "maxSize" => {
                self.max_size =
                    NonZeroU64::new(self.map_to_u64(&value, name_text, u64::MAX, diagnostics)?);
            }
            "ignore" => {
                self.ignore = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "ignoreUnknown" => {
                self.ignore_unknown = self.map_to_boolean(&value, name_text, diagnostics);
            }
            _ => {}
        }
        Some(())
    }
}
