use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachKey, SvelteEachKeyFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachKey;
impl FormatNodeRule<SvelteEachKey> for FormatSvelteEachKey {
    fn fmt_fields(&self, node: &SvelteEachKey, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachKeyFields {
            expression,
            l_paren_token,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                expression.format(),
                r_paren_token.format()
            ]
        )
    }
}
