use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteClassDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteClassDirective;
impl FormatNodeRule<SvelteClassDirective> for FormatSvelteClassDirective {
    fn fmt_fields(&self, node: &SvelteClassDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
