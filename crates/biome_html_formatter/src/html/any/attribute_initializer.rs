//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_formatter::FormatRuleWithOptions;
use biome_html_syntax::AnyHtmlAttributeInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyHtmlAttributeInitializer {
    compact: bool,
}

impl FormatRule<AnyHtmlAttributeInitializer> for FormatAnyHtmlAttributeInitializer {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnyHtmlAttributeInitializer, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnyHtmlAttributeInitializer::HtmlAttributeSingleTextExpression(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
            AnyHtmlAttributeInitializer::HtmlString(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
        }
    }
}

impl FormatRuleWithOptions<AnyHtmlAttributeInitializer> for FormatAnyHtmlAttributeInitializer {
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
