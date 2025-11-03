use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_html_syntax::SvelteBindingList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindingList;
impl FormatRule<SvelteBindingList> for FormatSvelteBindingList {
    type Context = HtmlFormatContext;
    fn fmt(&self, node: &SvelteBindingList, f: &mut HtmlFormatter) -> FormatResult<()> {
        let separator = space();
        let mut joiner = f.join_with(&separator);

        for formatted in node.format_separated(",") {
            joiner.entry(&formatted);
        }

        joiner.finish()
    }
}
