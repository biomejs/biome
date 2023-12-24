use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassNth, CssPseudoClassNthFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNth;
impl FormatNodeRule<CssPseudoClassNth> for FormatCssPseudoClassNth {
    fn fmt_fields(&self, node: &CssPseudoClassNth, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPseudoClassNthFields {
            sign,
            value,
            symbol_token,
            offset,
        } = node.as_fields();

        write!(f, [sign.format(), value.format(), symbol_token.format(),])?;

        if offset.is_some() {
            write!(f, [soft_line_break_or_space(), offset.format()])?;
        }

        Ok(())
    }
}
