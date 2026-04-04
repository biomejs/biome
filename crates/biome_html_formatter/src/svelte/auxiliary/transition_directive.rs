use crate::prelude::*;
use crate::utils::svelte_directive::FmtSvelteDirective;
use biome_html_syntax::SvelteTransitionDirective;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteTransitionDirective;
impl FormatNodeRule<SvelteTransitionDirective> for FormatSvelteTransitionDirective {
    fn fmt_fields(
        &self,
        node: &SvelteTransitionDirective,
        f: &mut HtmlFormatter,
    ) -> FormatResult<()> {
        FmtSvelteDirective::from(node).fmt(f)
    }
}
