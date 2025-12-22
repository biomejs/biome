use crate::prelude::*;
use biome_css_syntax::CssCustomIdentifierCommaSeparatedList;
use biome_formatter::FormatRuleWithOptions;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifierCommaSeparatedList {
    layout: CssCustomIdentifierLayout,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatCssCustomIdentifierCommaSeparatedListOptions {
    pub(crate) layout: CssCustomIdentifierLayout,
}

impl FormatCssCustomIdentifierCommaSeparatedListOptions {
    /// Sets the layout to format identifiers with soft line breaks (fluid layout).
    pub(crate) fn with_fluid_layout(mut self) -> Self {
        self.layout = CssCustomIdentifierLayout::Fluid;
        self
    }
}

/// Defines how a list of CSS custom identifiers should be formatted.
///
/// - [`OneLine`] — Formats all identifiers on a single line.
/// - [`Fluid`] — Formats identifiers with soft line breaks, allowing wrapping when needed.
///
/// ## Examples
///
/// ```css
/// /* OneLine */
/// :active-view-transition-type(value1, value2, value3)
///
/// /* Fluid */
/// :active-view-transition-type(value1, value2, value3)
/// :active-view-transition-type(value1,
///                              value2,
///                              value3,
///                              value4)
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum CssCustomIdentifierLayout {
    #[default]
    OneLine,
    Fluid,
}

impl FormatRuleWithOptions<CssCustomIdentifierCommaSeparatedList>
    for FormatCssCustomIdentifierCommaSeparatedList
{
    type Options = FormatCssCustomIdentifierCommaSeparatedListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options.layout;
        self
    }
}

impl FormatRule<CssCustomIdentifierCommaSeparatedList>
    for FormatCssCustomIdentifierCommaSeparatedList
{
    type Context = CssFormatContext;
    fn fmt(
        &self,
        node: &CssCustomIdentifierCommaSeparatedList,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        match self.layout {
            CssCustomIdentifierLayout::OneLine => {
                let separator = space();
                let mut joiner = f.join_with(&separator);
                for formatted in node.format_separated(",") {
                    joiner.entry(&formatted);
                }
                joiner.finish()
            }
            CssCustomIdentifierLayout::Fluid => {
                let separator = soft_line_break_or_space();
                let mut joiner = f.join_with(&separator);
                for formatted in node.format_separated(",") {
                    joiner.entry(&formatted);
                }
                joiner.finish()
            }
        }
    }
}
