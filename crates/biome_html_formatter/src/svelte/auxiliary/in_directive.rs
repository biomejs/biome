use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteInDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteInDirective;
impl FormatNodeRule<SvelteInDirective> for FormatSvelteInDirective {
    fn fmt_fields(&self, node: &SvelteInDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
