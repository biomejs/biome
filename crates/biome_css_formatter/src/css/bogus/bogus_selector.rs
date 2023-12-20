use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSelector;
impl FormatBogusNodeRule<CssBogusSelector> for FormatCssBogusSelector {}
