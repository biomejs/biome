use crate::prelude::*;
use biome_css_syntax::{ScssInterpolatedIdentifierHyphen, ScssInterpolatedIdentifierHyphenFields};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedIdentifierHyphen;
impl FormatNodeRule<ScssInterpolatedIdentifierHyphen> for FormatScssInterpolatedIdentifierHyphen {
    fn fmt_fields(
        &self,
        node: &ScssInterpolatedIdentifierHyphen,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let ScssInterpolatedIdentifierHyphenFields { minus_token } = node.as_fields();

        write![f, [minus_token.format()]]
    }
}
