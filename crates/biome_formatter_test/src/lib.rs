use biome_parser::AnyParse;
use biome_rowan::{Language, SyntaxNode, TextRange};
use rome_formatter::{
    CstFormatContext, FormatContext, FormatLanguage, FormatOptions, FormatResult, Formatted,
    Printed,
};

pub mod check_reformat;
pub mod diff_report;
pub mod snapshot_builder;
pub mod spec;
pub mod test_prettier_snapshot;
pub mod utils;

pub trait TestFormatLanguage {
    type SyntaxLanguage: Language + 'static;
    type Options: FormatOptions + std::fmt::Display + Clone;
    type Context: CstFormatContext<Options = Self::Options>;
    type FormatLanguage: FormatLanguage<Context = Self::Context, SyntaxLanguage = Self::SyntaxLanguage>
        + 'static;

    fn parse(&self, text: &str) -> AnyParse;

    fn deserialize_format_options(
        &self,
        options: &str,
    ) -> Vec<<Self::Context as FormatContext>::Options>;

    fn format_node(
        &self,
        options: Self::Options,
        node: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> FormatResult<Formatted<Self::Context>>;

    fn format_range(
        &self,
        options: Self::Options,
        node: &SyntaxNode<Self::SyntaxLanguage>,
        range: TextRange,
    ) -> FormatResult<Printed>;
}
