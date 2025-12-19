use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachIndex, SvelteEachIndexFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachIndex;
impl FormatNodeRule<SvelteEachIndex> for FormatSvelteEachIndex {
    fn fmt_fields(&self, node: &SvelteEachIndex, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachIndexFields { comma_token, value } = node.as_fields();

        write!(f, [comma_token.format(), space(), value.format()])
    }
}
