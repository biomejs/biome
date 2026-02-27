use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteAnimateDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteAnimateDirective;
impl FormatNodeRule<SvelteAnimateDirective> for FormatSvelteAnimateDirective {
    fn fmt_fields(&self, node: &SvelteAnimateDirective, f: &mut HtmlFormatter) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
