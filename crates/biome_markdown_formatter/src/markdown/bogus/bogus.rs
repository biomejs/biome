use crate::FormatBogusNodeRule;
use biome_markdown_syntax::MdBogus;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMdBogus;
impl FormatBogusNodeRule<MdBogus> for FormatMdBogus {}
