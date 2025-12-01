use crate::file_handlers::DocumentFileSource;
use crate::settings::{
    FormatSettings, LanguageListSettings, LanguageSettings, OverrideSettings, ServiceLanguage,
    Settings,
};
use biome_analyze::AnalyzerOptions;
use biome_fs::BiomePath;
use biome_yaml_formatter::YamlFormatOptions;
use biome_yaml_syntax::YamlLanguage;
use camino::Utf8Path;

impl ServiceLanguage for YamlLanguage {
    type FormatterSettings = ();
    type LinterSettings = ();
    type AssistSettings = ();
    type FormatOptions = YamlFormatOptions;
    type ParserSettings = ();
    type ParserOptions = ();
    type EnvironmentSettings = ();

    fn lookup_settings(_languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        todo!()
    }

    fn resolve_environment(_settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        todo!()
    }

    fn resolve_parse_options(
        _overrides: &OverrideSettings,
        _language: &Self::ParserSettings,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        todo!()
    }

    fn resolve_format_options(
        _global: &FormatSettings,
        _overrides: &OverrideSettings,
        _language: &Self::FormatterSettings,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        todo!()
    }

    fn resolve_analyzer_options(
        _global: &Settings,
        _language: &Self::LinterSettings,
        _environment: Option<&Self::EnvironmentSettings>,
        _path: &BiomePath,
        _file_source: &DocumentFileSource,
        _suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        todo!()
    }

    fn linter_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        todo!()
    }

    fn formatter_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        todo!()
    }

    fn assist_enabled_for_file_path(_settings: &Settings, _path: &Utf8Path) -> bool {
        todo!()
    }
}
