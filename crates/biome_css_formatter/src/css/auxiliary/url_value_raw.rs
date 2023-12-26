use crate::prelude::*;
use biome_css_syntax::{CssUrlValueRaw, CssUrlValueRawFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssUrlValueRaw;
impl FormatNodeRule<CssUrlValueRaw> for FormatCssUrlValueRaw {
    fn fmt_fields(&self, node: &CssUrlValueRaw, f: &mut CssFormatter) -> FormatResult<()> {
        let CssUrlValueRawFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
