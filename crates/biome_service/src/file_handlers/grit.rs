use crate::settings::ServiceLanguage;
use biome_grit_syntax::GritLanguage;

impl ServiceLanguage for GritLanguage {
    type FormatterSettings = ();
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = GritFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();
    fn lookup_settings(
        languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        todo!()
    }

    fn resolve_format_options(
        global: Option<&crate::settings::FormatSettings>,
        overrides: Option<&crate::settings::OverrideSettings>,
        language: Option<&Self::FormatterSettings>,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        todo!()
    }

    fn resolve_analyzer_options(
        global: Option<&crate::settings::Settings>,
        linter: Option<&crate::settings::LinterSettings>,
        overrides: Option<&crate::settings::OverrideSettings>,
        language: Option<&Self::LinterSettings>,
        path: &biome_fs::BiomePath,
        file_source: &super::DocumentFileSource,
    ) -> biome_analyze::AnalyzerOptions {
        todo!()
    }
}
