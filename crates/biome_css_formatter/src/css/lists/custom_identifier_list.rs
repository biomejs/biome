use crate::prelude::*;
use biome_css_syntax::CssCustomIdentifierList;
use biome_formatter::FormatRuleWithOptions;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomIdentifierList {
    layout: CssCustomIdentifierLayout,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct FormatCssCustomIdentifierListOptions {
    pub(crate) layout: CssCustomIdentifierLayout,
}

impl FormatCssCustomIdentifierListOptions {
    /// Sets the layout to format identifiers with soft line breaks (fluid layout).
    pub(crate) fn with_fluid_layout(mut self) -> Self {
        self.layout = CssCustomIdentifierLayout::Fluid;
        self
    }
}

/// Defines how a list of CSS custom identifiers should be formatted.
///
/// - [`OneLine`] — Formats all identifiers on a single line, separated by spaces.
/// - [`Fluid`] — Formats identifiers with soft line breaks, allowing wrapping when needed.
///
/// ## Examples
///
/// ```css
/// /* OneLine */
/// custom: value1 value2 value3;
///
/// /* Fluid */
/// custom: value1 value2 value3;
/// custom-long: value1
///              value2
///              value3
///              value4;
/// ```
#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum CssCustomIdentifierLayout {
    #[default]
    OneLine,
    Fluid,
}

impl FormatRuleWithOptions<CssCustomIdentifierList> for FormatCssCustomIdentifierList {
    type Options = FormatCssCustomIdentifierListOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options.layout;
        self
    }
}

impl FormatRule<CssCustomIdentifierList> for FormatCssCustomIdentifierList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &CssCustomIdentifierList, f: &mut CssFormatter) -> FormatResult<()> {
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
