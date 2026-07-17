use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{VueVForValue, VueVForValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForValue;
impl FormatNodeRule<VueVForValue> for FormatVueVForValue {
    fn fmt_fields(&self, node: &VueVForValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForValueFields {
            l_quote,
            binding,
            operator,
            expression,
            r_quote,
        } = node.as_fields();

        write!(
            f,
            [
                l_quote.format(),
                binding.format(),
                space(),
                operator.format(),
                space(),
                expression.format(),
                r_quote.format()
            ]
        )
    }
}
