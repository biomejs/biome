use crate::configuration::json::JsonConfiguration;
use crate::configuration::organize_imports::OrganizeImports;
use crate::configuration::parse::json::vcs::validate_vcs_configuration;
use crate::configuration::vcs::VcsConfiguration;
use crate::configuration::{
    FilesConfiguration, FormatterConfiguration, JavascriptConfiguration, LinterConfiguration,
};
use crate::Configuration;
use biome_deserialize::json::{has_only_known_keys, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_rowan::SyntaxNode;
use rome_json_syntax::{JsonLanguage, JsonSyntaxNode};

impl VisitJsonNode for Configuration {}

impl VisitNode<JsonLanguage> for Configuration {
    fn visit_member_name(
        &mut self,
        node: &JsonSyntaxNode,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<()> {
        has_only_known_keys(node, Configuration::KNOWN_KEYS, diagnostics)
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
            "$schema" => {
                self.schema = self.map_to_string(&value, name_text, diagnostics);
            }
            "files" => {
                let mut files = FilesConfiguration::default();
                self.map_to_object(&value, name_text, &mut files, diagnostics)?;
                self.files = Some(files);
            }
            "vcs" => {
                let mut vcs = VcsConfiguration::default();
                self.map_to_object(&value, name_text, &mut vcs, diagnostics)?;
                validate_vcs_configuration(&value, &mut vcs, diagnostics);
                self.vcs = Some(vcs);
            }
            "formatter" => {
                let mut formatter = FormatterConfiguration::default();
                self.map_to_object(&value, name_text, &mut formatter, diagnostics)?;
                self.formatter = Some(formatter);
            }
            "linter" => {
                let mut linter = LinterConfiguration::default();
                self.map_to_object(&value, name_text, &mut linter, diagnostics)?;
                self.linter = Some(linter);
            }
            "javascript" => {
                let mut javascript = JavascriptConfiguration::default();
                self.map_to_object(&value, name_text, &mut javascript, diagnostics)?;
                self.javascript = Some(javascript);
            }
            "json" => {
                let mut json = JsonConfiguration::default();
                self.map_to_object(&value, name_text, &mut json, diagnostics)?;
                self.json = Some(json);
            }
            "organizeImports" => {
                let mut organize_imports = OrganizeImports::default();
                self.map_to_object(&value, name_text, &mut organize_imports, diagnostics)?;
                self.organize_imports = Some(organize_imports);
            }
            "extends" => {
                self.extends = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            _ => {}
        }

        Some(())
    }
}
