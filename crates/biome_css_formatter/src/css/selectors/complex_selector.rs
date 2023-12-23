use crate::prelude::*;
use biome_css_syntax::{CssComplexSelector, CssComplexSelectorFields, CssSyntaxKind};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssComplexSelector;
impl FormatNodeRule<CssComplexSelector> for FormatCssComplexSelector {
    fn fmt_fields(&self, node: &CssComplexSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssComplexSelectorFields {
            left,
            combinator,
            right,
        } = node.as_fields();

        let combinator = combinator?;

        // Space combinators only care that there is _some_ whitespace between
        // the two selectors. Here, it gets replaced by a soft_line_break_or_space
        // to allow the complete selector to break onto multiple lines if needed.
        let formatted_combinator = format_with(|f| {
            if matches!(combinator.kind(), CssSyntaxKind::CSS_SPACE_LITERAL) {
                write!(f, [format_removed(&combinator)])
            } else {
                write!(f, [combinator.format()])
            }
        });

        write!(
            f,
            [
                left.format(),
                soft_line_break_or_space(),
                formatted_combinator,
                space(),
                right.format()
            ]
        )
    }
}
