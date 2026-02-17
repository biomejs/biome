//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_formatter::FormatRuleWithOptions;
use biome_html_syntax::AnySvelteBindingProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySvelteBindingProperty {
    /// Whether it should be formatted in compact mode. In compact mode, all tokens and children
    /// are removed
    pub compact: bool,
}
impl FormatRule<AnySvelteBindingProperty> for FormatAnySvelteBindingProperty {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AnySvelteBindingProperty, f: &mut HtmlFormatter) -> FormatResult<()> {
        match node {
            AnySvelteBindingProperty::SvelteLiteral(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
            AnySvelteBindingProperty::SvelteName(node) => {
                node.format().with_options(self.compact).fmt(f)
            }
        }
    }
}

impl FormatRuleWithOptions<AnySvelteBindingProperty> for FormatAnySvelteBindingProperty {
    type Options = bool;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.compact = options;
        self
    }
}
