use crate::prelude::*;
use biome_html_syntax::AngularForLetBindingList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAngularForLetBindingList;
impl FormatRule<AngularForLetBindingList> for FormatAngularForLetBindingList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &AngularForLetBindingList, f: &mut HtmlFormatter) -> FormatResult<()> {
        format_html_verbatim_node(node.syntax()).fmt(f)
    }
}
