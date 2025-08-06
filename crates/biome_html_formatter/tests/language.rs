use biome_formatter_test::TestFormatLanguage;
use biome_fs::BiomePath;
use biome_html_formatter::HtmlFormatLanguage;
use biome_html_formatter::context::HtmlFormatContext;
use biome_html_parser::{HtmlParseOptions, parse_html};
use biome_html_syntax::{HtmlFileSource, HtmlLanguage};
use biome_parser::AnyParse;
use biome_service::{
    settings::{ServiceLanguage, Settings},
    workspace::DocumentFileSource,
};

pub struct HtmlTestFormatLanguage {
    source_type: HtmlFileSource,
}

impl HtmlTestFormatLanguage {
    pub fn new(source_type: HtmlFileSource) -> Self {
        Self { source_type }
    }
}

impl TestFormatLanguage for HtmlTestFormatLanguage {
    type ServiceLanguage = HtmlLanguage;
    type Context = HtmlFormatContext;
    type FormatLanguage = HtmlFormatLanguage;

    /// Parses HTML text into a generic parse representation using options derived from the file source.
    ///
    /// # Examples
    ///
    /// ```
    /// let language = HtmlTestFormatLanguage::new(HtmlFileSource::default());
    /// let parsed = language.parse("<div>Hello</div>");
    /// assert!(parsed.syntax().is_some());
    /// ```
    fn parse(&self, text: &str) -> AnyParse {
        parse_html(text, HtmlParseOptions::from(&self.source_type)).into()
    }

    /// Creates an `HtmlFormatLanguage` instance with formatting options resolved from the provided settings and file source.
    ///
    /// The formatting options are determined by combining global, override, and HTML-specific formatter settings, along with the given document file source.
    ///
    /// # Examples
    ///
    /// ```
    /// let language = HtmlTestFormatLanguage::new(source_type);
    /// let format_language = language.to_format_language(&settings, &file_source);
    /// ```
    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage {
        let options = Self::ServiceLanguage::resolve_format_options(
            &settings.formatter,
            &settings.override_settings,
            &settings.languages.html.formatter,
            &BiomePath::new(""),
            file_source,
        );
        HtmlFormatLanguage::new(options)
    }
}
