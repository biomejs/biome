use biome_html_formatter::HtmlFormatOptions;
use biome_html_syntax::HtmlLanguage;

use crate::settings::ServiceLanguage;

impl ServiceLanguage for HtmlLanguage {
    type FormatterSettings = ();
    type LinterSettings = ();
    type OrganizeImportsSettings = ();
    type FormatOptions = HtmlFormatOptions;
    type ParserSettings = ();
    type EnvironmentSettings = ();

    fn lookup_settings(
        _languages: &crate::settings::LanguageListSettings,
    ) -> &crate::settings::LanguageSettings<Self> {
        todo!()
    }

    fn resolve_format_options(
        _global: Option<&crate::settings::FormatSettings>,
        _overrides: Option<&crate::settings::OverrideSettings>,
        _language: Option<&Self::FormatterSettings>,
        _path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> Self::FormatOptions {
        // TODO: actually resolve options
        HtmlFormatOptions::default()
    }

    fn resolve_analyzer_options(
        _global: Option<&crate::settings::Settings>,
        _linter: Option<&crate::settings::LinterSettings>,
        _overrides: Option<&crate::settings::OverrideSettings>,
        _language: Option<&Self::LinterSettings>,
        _path: &biome_fs::BiomePath,
        _file_source: &super::DocumentFileSource,
    ) -> biome_analyze::AnalyzerOptions {
        todo!()
    }
}
