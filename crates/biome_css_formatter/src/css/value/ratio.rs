use crate::prelude::*;
use biome_css_syntax::{CssRatio, CssRatioFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRatio;
impl FormatNodeRule<CssRatio> for FormatCssRatio {
    fn fmt_fields(&self, node: &CssRatio, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRatioFields {
            numerator,
            slash_token,
            denominator,
        } = node.as_fields();

        let numbers = format_with(|f| {
            let separator = soft_line_break_or_space();
            let mut filler = f.fill();

            filler.entry(&separator, &numerator.format());
            filler.entry(&separator, &slash_token.format());
            filler.entry(&separator, &denominator.format());
            filler.finish()
        });
        write!(f, [group(&numbers)])
    }
}
