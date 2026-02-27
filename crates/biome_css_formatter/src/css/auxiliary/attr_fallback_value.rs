use crate::prelude::*;
use biome_css_syntax::{CssAttrFallbackValue, CssAttrFallbackValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrFallbackValue;

impl FormatNodeRule<CssAttrFallbackValue> for FormatCssAttrFallbackValue {
    fn fmt_fields(&self, node: &CssAttrFallbackValue, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttrFallbackValueFields { comma_token, value } = node.as_fields();

        write!(
            f,
            [
                comma_token.format(),
                soft_line_break_or_space(),
                value.format()
            ]
        )
    }
}
