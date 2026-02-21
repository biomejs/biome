use crate::prelude::*;
use biome_css_syntax::{CssSyntaxMultiplier, CssSyntaxMultiplierFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSyntaxMultiplier;

impl FormatNodeRule<CssSyntaxMultiplier> for FormatCssSyntaxMultiplier {
    fn fmt_fields(&self, node: &CssSyntaxMultiplier, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSyntaxMultiplierFields { multiplier } = node.as_fields();
        write!(f, [multiplier.format()])
    }
}
