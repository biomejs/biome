use crate::{FormatBogusNodeRule, has_svelte_closing_delimiter, prelude::*};
use biome_formatter::FormatError;
use biome_html_syntax::SvelteBogusBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBogusBlock;
impl FormatBogusNodeRule<SvelteBogusBlock> for FormatSvelteBogusBlock {
    fn fmt(&self, node: &SvelteBogusBlock, f: &mut HtmlFormatter) -> FormatResult<()> {
        if !has_svelte_closing_delimiter(node.syntax()) {
            // Without a closing delimiter even the EOF trivia can belong to the block.
            return Err(FormatError::SyntaxError);
        }
        format_bogus_node(node.syntax()).fmt(f)
    }
}
