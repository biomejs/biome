use crate::prelude::*;
use biome_html_syntax::SvelteBindingList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindingList;
impl FormatRule<SvelteBindingList> for FormatSvelteBindingList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteBindingList, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
