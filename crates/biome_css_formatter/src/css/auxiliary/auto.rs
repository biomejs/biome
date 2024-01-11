use crate::prelude::*;
use biome_css_syntax::{CssAuto, CssAutoFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAuto;
impl FormatNodeRule<CssAuto> for FormatCssAuto {
    fn fmt_fields(&self, node: &CssAuto, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAutoFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
