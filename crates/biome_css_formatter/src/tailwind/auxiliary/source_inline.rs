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

        let should_insert_space = f.options().delimiter_spacing().value();

        if should_insert_space {
            write!(
                f,
                [
                    inline_token.format(),
                    l_paren_token.format(),
                    space(),
                    &content.format(),
                    space(),
                    r_paren_token.format(),
                ]
            )
        } else {
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
}
