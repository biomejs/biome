use crate::prelude::*;
use biome_html_syntax::VueVForOfOperator;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForOfOperator;
impl FormatNodeRule<VueVForOfOperator> for FormatVueVForOfOperator {
    fn fmt_fields(&self, node: &VueVForOfOperator, f: &mut HtmlFormatter) -> FormatResult<()> {
        node.of_token().format().fmt(f)
    }
}
