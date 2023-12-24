use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusSubSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSubSelector;
impl FormatBogusNodeRule<CssBogusSubSelector> for FormatCssBogusSubSelector {}
