use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteTemplateAttributeValue, SvelteTemplateAttributeValueFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteTemplateAttributeValue;
impl FormatNodeRule<SvelteTemplateAttributeValue> for FormatSvelteTemplateAttributeValue {
    fn fmt_fields(
        &self,
        node: &SvelteTemplateAttributeValue,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteTemplateAttributeValueFields {
            l_quote,
            elements,
            r_quote,
        } = node.as_fields();
        write!(f, [l_quote.format(), elements.format(), r_quote.format()])
    }
}
