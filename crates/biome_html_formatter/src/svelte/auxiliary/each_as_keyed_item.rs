use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachAsKeyedItem, SvelteEachAsKeyedItemFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachAsKeyedItem;
impl FormatNodeRule<SvelteEachAsKeyedItem> for FormatSvelteEachAsKeyedItem {
    fn fmt_fields(&self, node: &SvelteEachAsKeyedItem, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachAsKeyedItemFields {
            as_token,
            name,
            index,
            key,
        } = node.as_fields();

        write!(f, [space(), as_token.format(), space(), name.format(),])?;

        if let Some(index) = index {
            write!(f, [index.format()])?;
        }

        if let Some(key) = key {
            write!(f, [space(), key.format()])?;
        }

        Ok(())
    }
}
