use crate::FormatBogusNodeRule;
use biome_markdown_syntax::MdBogusBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBogusBlock;
impl FormatBogusNodeRule<MdBogusBlock> for FormatMdBogusBlock {}
