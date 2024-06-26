use crate::prelude::*;
use biome_css_syntax::{CssUnicodeRangeInterval, CssUnicodeRangeIntervalFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnicodeRangeInterval;
impl FormatNodeRule<CssUnicodeRangeInterval> for FormatCssUnicodeRangeInterval {
    fn fmt_fields(&self, node: &CssUnicodeRangeInterval, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnicodeRangeIntervalFields {
            start,
            minus_token,
            end,
        } = node.as_fields();
        write!(f, [start.format(), minus_token.format(), end.format()])
    }
}
