use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteOutDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteOutDirective;
impl FormatNodeRule<SvelteOutDirective> for FormatSvelteOutDirective {
    fn fmt_fields(&self, node: &SvelteOutDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
