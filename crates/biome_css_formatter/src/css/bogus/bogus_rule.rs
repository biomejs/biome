use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusRule;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusRule;
impl FormatBogusNodeRule<CssBogusRule> for FormatCssBogusRule {}
