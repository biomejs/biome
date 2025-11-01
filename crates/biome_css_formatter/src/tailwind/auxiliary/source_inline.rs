use crate::prelude::*;
use biome_css_syntax::{TwSourceInline, TwSourceInlineFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwSourceInline;
impl FormatNodeRule<TwSourceInline> for FormatTwSourceInline {
    fn fmt_fields(&self, node: &TwSourceInline, f: &mut CssFormatter) -> FormatResult<()> {
        let TwSourceInlineFields {
            inline_token,
            l_paren_token,
            content,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                inline_token.format(),
                l_paren_token.format(),
                &content.format(),
                r_paren_token.format(),
            ]
        )
    }
}
