use crate::prelude::*;
use biome_html_syntax::VueModifierList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueModifierList;
impl FormatRule<VueModifierList> for FormatVueModifierList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &VueModifierList, f: &mut HtmlFormatter) -> FormatResult<()> {
        f.join().entries(node.iter().formatted()).finish()
    }
}
