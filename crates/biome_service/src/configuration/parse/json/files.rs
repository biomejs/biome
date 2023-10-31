use crate::configuration::FilesConfiguration;
use biome_deserialize::json::{report_unknown_map_key, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_json_syntax::JsonLanguage;
use biome_rowan::SyntaxNode;
use std::num::NonZeroU64;

impl FilesConfiguration {
    const ALLOWED_KEYS: &'static [&'static str] =
        &["maxSize", "ignore", "include", "ignoreUnknown"];
}

impl VisitNode<JsonLanguage> for FilesConfiguration {
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
            "maxSize" => {
                self.max_size =
                    NonZeroU64::new(self.map_to_u64(&value, name_text, u64::MAX, diagnostics)?);
            }
            "ignore" => {
                self.ignore = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }

            "include" => {
                self.include = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "ignoreUnknown" => {
                self.ignore_unknown = self.map_to_boolean(&value, name_text, diagnostics);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }
        Some(())
    }
}
