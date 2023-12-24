use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassNthNumber, CssPseudoClassNthNumberFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassNthNumber;
impl FormatNodeRule<CssPseudoClassNthNumber> for FormatCssPseudoClassNthNumber {
    fn fmt_fields(&self, node: &CssPseudoClassNthNumber, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPseudoClassNthNumberFields { sign, value } = node.as_fields();

        write!(f, [sign.format(), value.format()])
    }
}
