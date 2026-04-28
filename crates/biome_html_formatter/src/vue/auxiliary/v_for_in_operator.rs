use crate::prelude::*;
use biome_html_syntax::VueVForInOperator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForInOperator;
impl FormatNodeRule<VueVForInOperator> for FormatVueVForInOperator {
    fn fmt_fields(&self, node: &VueVForInOperator, f: &mut HtmlFormatter) -> FormatResult<()> {
        node.in_token().format().fmt(f)
    }
}
