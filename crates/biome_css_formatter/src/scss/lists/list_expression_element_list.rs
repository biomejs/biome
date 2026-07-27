use crate::prelude::*;
use biome_css_syntax::ScssListExpressionElementList;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::{FormatRuleWithOptions, write};
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssListExpressionElementList {
    layout: ScssListElementLayout,
}

/// Controls separators between SCSS comma-list elements.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub(crate) enum ScssListElementLayout {
    /// Allows separators to break with the enclosing group.
    #[default]
    Flexible,

    /// Preserves source line breaks between elements.
    PreserveSourceBreaks,
}

impl FormatRuleWithOptions<ScssListExpressionElementList> for FormatScssListExpressionElementList {
    type Options = ScssListElementLayout;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.layout = options;
        self
    }
}

impl FormatRule<ScssListExpressionElementList> for FormatScssListExpressionElementList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssListExpressionElementList, f: &mut CssFormatter) -> FormatResult<()> {
        let separated = node
            .format_separated(",")
            .with_trailing_separator(TrailingSeparator::Omit);

        for (index, (element, formatted)) in node.elements().zip(separated).enumerate() {
            if index > 0 {
                if self.layout == ScssListElementLayout::PreserveSourceBreaks
                    && element
                        .node()
                        .is_ok_and(|element| element.syntax().has_leading_newline())
                {
                    write!(f, [hard_line_break()])?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }
            }
            write!(f, [formatted])?;
        }

        Ok(())
    }
}
