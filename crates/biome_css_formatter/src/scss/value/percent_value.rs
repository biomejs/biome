use crate::prelude::*;
use biome_css_syntax::{ScssPercentValue, ScssPercentValueFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssPercentValue;
impl FormatNodeRule<ScssPercentValue> for FormatScssPercentValue {
    fn fmt_fields(&self, node: &ScssPercentValue, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssPercentValueFields { percent_token } = node.as_fields();

        write!(f, [percent_token.format()])
    }
}
