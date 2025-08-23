use crate::prelude::*;
use biome_css_syntax::{TwCustomVariantShorthand, TwCustomVariantShorthandFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwCustomVariantShorthand;
impl FormatNodeRule<TwCustomVariantShorthand> for FormatTwCustomVariantShorthand {
    fn fmt_fields(
        &self,
        node: &TwCustomVariantShorthand,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let TwCustomVariantShorthandFields {
            l_paren_token,
            selector,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_paren_token.format(),
                selector.format(),
                r_paren_token.format(),
                semicolon_token.format()
            ]
        )
    }
}
