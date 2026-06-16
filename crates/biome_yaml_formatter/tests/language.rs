use biome_formatter::{IndentWidth, LineEnding, LineWidth};
use biome_formatter_test::TestFormatLanguage;
use biome_languages::{DocumentFileSource, yaml::YamlFileSource};
use biome_parser::AnyParsedSource;
use biome_service::settings::Settings;
use biome_yaml_formatter::{YamlFormatContext, YamlFormatLanguage, YamlFormatOptions};
use biome_yaml_parser::parse_yaml;
use biome_yaml_syntax::YamlLanguage;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct YamlTestFormatLanguage {
    _source_type: YamlFileSource,
}

impl TestFormatLanguage for YamlTestFormatLanguage {
    type ServiceLanguage = YamlLanguage;
    type Context = YamlFormatContext;
    type FormatLanguage = YamlFormatLanguage;

    fn parse(&self, text: &str) -> AnyParsedSource {
        parse_yaml(text).into()
    }

    fn to_format_language(
        &self,
        _settings: &Settings,
        _file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        // let language_settings = &settings.languages.yaml.formatter;
        // let options = Self::ServiceLanguage::resolve_format_options(
        //     &settings.formatter,
        //     &settings.override_settings,
        //     language_settings,
        //     &BiomePath::new(""),
        //     file_source,
        // );
        YamlFormatLanguage::new(YamlFormatOptions::default())
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum YamlSerializableLineEnding {
    ///  Line Feed only (\n), common on Linux and macOS as well as inside git repos
    Lf,

    /// Carriage Return + Line Feed characters (\r\n), common on Windows
    Crlf,

    /// Carriage Return character only (\r), used very rarely
    Cr,
}

impl From<YamlSerializableLineEnding> for LineEnding {
    fn from(test: YamlSerializableLineEnding) -> Self {
        match test {
            YamlSerializableLineEnding::Lf => Self::Lf,
            YamlSerializableLineEnding::Crlf => Self::Crlf,
            YamlSerializableLineEnding::Cr => Self::Cr,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct YamlSerializableFormatOptions {
    /// The indent width.
    pub indent_width: Option<u8>,

    /// The type of line ending.
    pub line_ending: Option<YamlSerializableLineEnding>,

    /// What's the max width of a line. Defaults to 80.
    pub line_width: Option<u16>,
}

impl From<YamlSerializableFormatOptions> for YamlFormatOptions {
    fn from(test: YamlSerializableFormatOptions) -> Self {
        Self::default()
            .with_indent_width(
                test.indent_width
                    .and_then(|width| IndentWidth::try_from(width).ok())
                    .unwrap_or_default(),
            )
            .with_line_width(
                test.line_width
                    .and_then(|width| LineWidth::try_from(width).ok())
                    .unwrap_or_default(),
            )
    }
}
