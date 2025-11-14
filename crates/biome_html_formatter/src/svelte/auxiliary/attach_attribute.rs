use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteAttachAttribute, SvelteAttachAttributeFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAttachAttribute;
impl FormatNodeRule<SvelteAttachAttribute> for FormatSvelteAttachAttribute {
    fn fmt_fields(&self, node: &SvelteAttachAttribute, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteAttachAttributeFields {
            sv_curly_at_token,
            expression,
            r_curly_token,
            attach_token,
        } = node.as_fields();

        write!(
            f,
            [
                sv_curly_at_token.format(),
                attach_token.format(),
                expression.format(),
                r_curly_token.format()
            ]
        )
    }
}
