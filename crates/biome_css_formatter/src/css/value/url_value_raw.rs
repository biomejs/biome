use crate::prelude::*;
use biome_css_syntax::{CssUrlValueRaw, CssUrlValueRawFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlValueRaw;
impl FormatNodeRule<CssUrlValueRaw> for FormatCssUrlValueRaw {
    fn fmt_fields(&self, node: &CssUrlValueRaw, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUrlValueRawFields { value_token } = node.as_fields();
        let value_token = value_token?;
        let text = value_token.token_text();
        write!(
            f,
            [format_replaced(
                &value_token,
                &dynamic_text(text.trim(), value_token.text_trimmed_range().start())
            )]
        )
    }
}
