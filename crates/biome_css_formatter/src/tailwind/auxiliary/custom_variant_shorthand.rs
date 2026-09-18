use crate::prelude::*;
use biome_css_syntax::{TwCustomVariantShorthand, TwCustomVariantShorthandFields};
use biome_formatter::{format_args, write};

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

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&selector.format(), should_insert_space),
                    r_paren_token.format()
                ]),
                semicolon_token.format()
            ]
        )
    }
}
