use crate::prelude::*;
use biome_css_syntax::{CssGritMetavariable, CssGritMetavariableFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGritMetavariable;
impl FormatNodeRule<CssGritMetavariable> for FormatCssGritMetavariable {
    fn fmt_fields(&self, node: &CssGritMetavariable, f: &mut CssFormatter) -> FormatResult<()> {
        let CssGritMetavariableFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
