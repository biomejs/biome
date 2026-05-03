use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteSquareDestructuredName, SvelteSquareDestructuredNameFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteSquareDestructuredName;
impl FormatNodeRule<SvelteSquareDestructuredName> for FormatSvelteSquareDestructuredName {
    fn fmt_fields(
        &self,
        node: &SvelteSquareDestructuredName,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteSquareDestructuredNameFields {
            l_brack_token,
            r_brack_token,
            names,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                space(),
                names.format(),
                space(),
                r_brack_token.format(),
            ]
        )
    }
}
