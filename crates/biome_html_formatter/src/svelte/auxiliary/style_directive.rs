use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteStyleDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteStyleDirective;
impl FormatNodeRule<SvelteStyleDirective> for FormatSvelteStyleDirective {
    fn fmt_fields(&self, node: &SvelteStyleDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
