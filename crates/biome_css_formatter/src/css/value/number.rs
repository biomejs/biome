use crate::prelude::*;
use biome_css_syntax::{CssNumber, CssNumberFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNumber;
impl FormatNodeRule<CssNumber> for FormatCssNumber {
    fn fmt_fields(&self, node: &CssNumber, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNumberFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
