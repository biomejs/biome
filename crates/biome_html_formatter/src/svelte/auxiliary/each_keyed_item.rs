use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteEachKeyedItem, SvelteEachKeyedItemFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteEachKeyedItem;
impl FormatNodeRule<SvelteEachKeyedItem> for FormatSvelteEachKeyedItem {
    fn fmt_fields(&self, node: &SvelteEachKeyedItem, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteEachKeyedItemFields { index } = node.as_fields();

        if let Some(index) = index {
            write!(f, [index.format()])
        } else {
            Ok(())
        }
    }
}
