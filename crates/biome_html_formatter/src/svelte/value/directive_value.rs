use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteDirectiveValue, SvelteDirectiveValueFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDirectiveValue;
impl FormatNodeRule<SvelteDirectiveValue> for FormatSvelteDirectiveValue {
    fn fmt_fields(&self, node: &SvelteDirectiveValue, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteDirectiveValueFields {
            property,
            colon_token,
            modifiers,
            initializer,
        } = node.as_fields();

        write!(
            f,
            [colon_token.format(), property.format(), modifiers.format()]
        )?;

        if let Some(initializer) = initializer {
            write!(f, [initializer.format()])?;
        }

        Ok(())
    }
}
