use crate::prelude::*;
use biome_css_syntax::{ScssColonValue, ScssColonValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssColonValue;
impl FormatNodeRule<ScssColonValue> for FormatScssColonValue {
    fn fmt_fields(&self, node: &ScssColonValue, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssColonValueFields { colon_token } = node.as_fields();

        write!(f, [colon_token.format()])
    }
}
