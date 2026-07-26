use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteLetDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteLetDirective;
impl FormatNodeRule<SvelteLetDirective> for FormatSvelteLetDirective {
    fn fmt_fields(&self, node: &SvelteLetDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
