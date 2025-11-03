use crate::prelude::*;
use biome_formatter::write;
use biome_html_syntax::{SvelteDebugBlock, SvelteDebugBlockFields};
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteDebugBlock;
impl FormatNodeRule<SvelteDebugBlock> for FormatSvelteDebugBlock {
    fn fmt_fields(&self, node: &SvelteDebugBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        let SvelteDebugBlockFields {
            sv_curly_at_token,
            debug_token,
            bindings,
            r_curly_token,
        } = node.as_fields();

        write!(f, [sv_curly_at_token.format(), debug_token.format(),])?;

        if !bindings.is_empty() {
            write!(f, [space()])?;
        }

        write!(f, [bindings.format()])?;

        write!(f, [r_curly_token.format()])
    }
}
