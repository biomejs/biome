use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteUseDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteUseDirective;
impl FormatNodeRule<SvelteUseDirective> for FormatSvelteUseDirective {
    fn fmt_fields(&self, node: &SvelteUseDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
