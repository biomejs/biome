use crate::prelude::*;
use biome_css_syntax::{ScssKeywordArgument, ScssKeywordArgumentFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssKeywordArgument;
impl FormatNodeRule<ScssKeywordArgument> for FormatScssKeywordArgument {
    fn fmt_fields(&self, node: &ScssKeywordArgument, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssKeywordArgumentFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )
    }
}
