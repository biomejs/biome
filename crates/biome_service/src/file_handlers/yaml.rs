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

    fn lookup_settings(languages: &LanguageListSettings) -> &LanguageSettings<Self> {
        todo!()
    }

    fn resolve_environment(settings: &Settings) -> Option<&Self::EnvironmentSettings> {
        todo!()
    }

    fn resolve_parse_options(
        overrides: &OverrideSettings,
        language: &Self::ParserSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::ParserOptions {
        todo!()
    }

    fn resolve_format_options(
        global: &FormatSettings,
        overrides: &OverrideSettings,
        language: &Self::FormatterSettings,
        path: &BiomePath,
        file_source: &DocumentFileSource,
    ) -> Self::FormatOptions {
        todo!()
    }

    fn resolve_analyzer_options(
        global: &Settings,
        language: &Self::LinterSettings,
        environment: Option<&Self::EnvironmentSettings>,
        path: &BiomePath,
        file_source: &DocumentFileSource,
        suppression_reason: Option<&str>,
    ) -> AnalyzerOptions {
        todo!()
    }

    fn linter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        todo!()
    }

    fn formatter_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        todo!()
    }

    fn assist_enabled_for_file_path(settings: &Settings, path: &Utf8Path) -> bool {
        todo!()
    }
}
