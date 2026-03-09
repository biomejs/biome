use crate::prelude::*;
use biome_css_syntax::{ScssMapExpressionPair, ScssMapExpressionPairFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMapExpressionPair;
impl FormatNodeRule<ScssMapExpressionPair> for FormatScssMapExpressionPair {
    fn fmt_fields(&self, node: &ScssMapExpressionPair, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMapExpressionPairFields {
            key,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [key.format(), colon_token.format(), space(), value.format()]
        )
    }
}
