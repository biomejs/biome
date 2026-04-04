use crate::{
    css::lists::custom_identifier_comma_separated_list::CssCustomIdentifierLayout, prelude::*,
};
use biome_css_syntax::CssCustomIdentifierSpaceSeparatedList;
use biome_formatter::FormatRuleWithOptions;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifierSpaceSeparatedList {
    layout: CssCustomIdentifierLayout,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatCssCustomIdentifierSpaceSeparatedListOptions {
    pub(crate) layout: CssCustomIdentifierLayout,
}

impl FormatRuleWithOptions<CssCustomIdentifierSpaceSeparatedList>
    for FormatCssCustomIdentifierSpaceSeparatedList
{
    type Options = FormatCssCustomIdentifierSpaceSeparatedListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options.layout;
        self
    }
}

impl FormatRule<CssCustomIdentifierSpaceSeparatedList>
    for FormatCssCustomIdentifierSpaceSeparatedList
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &CssCustomIdentifierSpaceSeparatedList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match self.layout {
            CssCustomIdentifierLayout::OneLine => f
                .join_with(&space())
                .entries(node.iter().formatted())
                .finish(),
            CssCustomIdentifierLayout::Fluid => f
                .join_with(&soft_line_break_or_space())
                .entries(node.iter().formatted())
                .finish(),
        }
    }
}
