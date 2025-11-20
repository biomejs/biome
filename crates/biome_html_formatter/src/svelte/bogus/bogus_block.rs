use crate::FormatBogusNodeRule;
use biome_html_syntax::SvelteBogusBlock;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSvelteBogusBlock;
impl FormatBogusNodeRule<SvelteBogusBlock> for FormatSvelteBogusBlock {}
