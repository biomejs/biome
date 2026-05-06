use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteBindDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBindDirective;
impl FormatNodeRule<SvelteBindDirective> for FormatSvelteBindDirective {
    fn fmt_fields(&self, node: &SvelteBindDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
