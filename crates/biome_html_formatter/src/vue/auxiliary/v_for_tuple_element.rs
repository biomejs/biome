use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForTupleElement, VueVForTupleElementFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForTupleElement;
impl FormatNodeRule<VueVForTupleElement> for FormatVueVForTupleElement {
    fn fmt_fields(&self, node: &VueVForTupleElement, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForTupleElementFields {
            comma_token,
            binding,
        } = node.as_fields();

        write!(f, [comma_token.format(), space(), binding.format()])
    }
}
