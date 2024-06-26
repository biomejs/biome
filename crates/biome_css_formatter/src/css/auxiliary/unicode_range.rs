use crate::prelude::*;
use biome_css_syntax::{CssUnicodeRange, CssUnicodeRangeFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUnicodeRange;
impl FormatNodeRule<CssUnicodeRange> for FormatCssUnicodeRange {
    fn fmt_fields(&self, node: &CssUnicodeRange, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUnicodeRangeFields {
            prefix_token,
            value,
        } = node.as_fields();

        write!(f, [prefix_token.format(), value.format()])
    }
}
