use crate::prelude::*;
use biome_html_syntax::SvelteDirectiveModifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveModifierList;
impl FormatRule<SvelteDirectiveModifierList> for FormatSvelteDirectiveModifierList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteDirectiveModifierList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
