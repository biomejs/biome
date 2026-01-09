use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteDirectiveModifier, SvelteDirectiveModifierFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveModifier;
impl FormatNodeRule<SvelteDirectiveModifier> for FormatSvelteDirectiveModifier {
    fn fmt_fields(
        &self,
        node: &SvelteDirectiveModifier,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        let SvelteDirectiveModifierFields {
            bitwise_or_token,
            name,
        } = node.as_fields();

        write!(f, [bitwise_or_token.format(), name.format()])
    }
}
