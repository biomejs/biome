use biome_formatter::{CstFormatContext, FormatLanguage, FormatResult, Formatted, Printed};
use biome_parser::AnyParse;
use biome_rowan::{SyntaxNode, TextRange};
use biome_service::file_handlers::DocumentFileSource;
use biome_service::settings::{ServiceLanguage, Settings};

pub mod check_reformat;
pub mod diff_report;
pub mod snapshot_builder;
pub mod spec;
pub mod test_prettier_snapshot;
pub mod utils;

pub trait TestFormatLanguage {
    type ServiceLanguage: ServiceLanguage + 'static;
    type Context: CstFormatContext<
        Options = <Self::ServiceLanguage as ServiceLanguage>::FormatOptions,
    >;
    type FormatLanguage: FormatLanguage<Context = Self::Context, SyntaxLanguage = Self::ServiceLanguage>
        + 'static
        + Clone;

    fn parse(&self, text: &str) -> AnyParse;

    fn format_node(
        &self,
        language: Self::FormatLanguage,
        node: &SyntaxNode<Self::ServiceLanguage>,
    ) -> FormatResult<Formatted<Self::Context>> {
        biome_formatter::format_node(node, language)
    }

    fn format_range(
        &self,
        language: Self::FormatLanguage,
        node: &SyntaxNode<Self::ServiceLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed> {
        biome_formatter::format_range(node, range, language)
    }

    fn to_format_language(
        &self,
        settings: &Settings,
        file_source: &DocumentFileSource,
    ) -> Self::FormatLanguage;
}
