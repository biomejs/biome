use crate::prelude::*;
use biome_css_syntax::{CssRatio, CssRatioFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRatio;
impl FormatNodeRule<CssRatio> for FormatCssRatio {
    fn fmt_fields(&self, node: &CssRatio, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRatioFields {
            numerator,
            slash_token,
            denominator,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                numerator.format(),
                space(),
                slash_token.format(),
                soft_line_break_or_space(),
                denominator.format()
            ])]
        )
    }
}
