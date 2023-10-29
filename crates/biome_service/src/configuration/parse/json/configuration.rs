use crate::configuration::json::JsonConfiguration;
use crate::configuration::organize_imports::OrganizeImports;
use crate::configuration::overrides::Overrides;
use crate::configuration::parse::json::vcs::validate_vcs_configuration;
use crate::configuration::vcs::VcsConfiguration;
use crate::configuration::{
    FilesConfiguration, FormatterConfiguration, JavascriptConfiguration, LinterConfiguration,
};
use crate::Configuration;
use biome_deserialize::json::{report_unknown_map_key, VisitJsonNode};
use biome_deserialize::{DeserializationDiagnostic, StringSet, VisitNode};
use biome_json_syntax::JsonLanguage;
use biome_rowan::SyntaxNode;

impl Configuration {
    const ALLOWED_KEYS: &'static [&'static str] = &[
        "vcs",
        "files",
        "linter",
        "formatter",
        "javascript",
        "json",
        "$schema",
        "organizeImports",
        "extends",
        "overrides",
    ];
}

impl VisitNode<JsonLanguage> for Configuration {
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
            "$schema" => {
                self.schema = self.map_to_string(&value, name_text, diagnostics);
            }
            "files" => {
                let mut files = FilesConfiguration::default();
                files.map_to_object(&value, name_text, diagnostics)?;
                self.files = Some(files);
            }
            "vcs" => {
                let mut vcs = VcsConfiguration::default();
                vcs.map_to_object(&value, name_text, diagnostics)?;
                validate_vcs_configuration(&value, &mut vcs, diagnostics);
                self.vcs = Some(vcs);
            }
            "formatter" => {
                let mut formatter = FormatterConfiguration::default();
                formatter.map_to_object(&value, name_text, diagnostics)?;
                self.formatter = Some(formatter);
            }
            "linter" => {
                let mut linter = LinterConfiguration::default();
                linter.map_to_object(&value, name_text, diagnostics)?;
                self.linter = Some(linter);
            }
            "javascript" => {
                let mut javascript = JavascriptConfiguration::default();
                javascript.map_to_object(&value, name_text, diagnostics)?;
                self.javascript = Some(javascript);
            }
            "json" => {
                let mut json = JsonConfiguration::default();
                json.map_to_object(&value, name_text, diagnostics)?;
                self.json = Some(json);
            }
            "organizeImports" => {
                let mut organize_imports = OrganizeImports::default();
                organize_imports.map_to_object(&value, name_text, diagnostics)?;
                self.organize_imports = Some(organize_imports);
            }
            "extends" => {
                self.extends = self
                    .map_to_index_set_string(&value, name_text, diagnostics)
                    .map(StringSet::new);
            }
            "overrides" => {
                let mut overrides = Overrides::default();
                overrides.map_to_array(&value, name_text, diagnostics)?;
                self.overrides = Some(overrides);
            }
            _ => {
                report_unknown_map_key(&name, Self::ALLOWED_KEYS, diagnostics);
            }
        }

        Some(())
    }
}
