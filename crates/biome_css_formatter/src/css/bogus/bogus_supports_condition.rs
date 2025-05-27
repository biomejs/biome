use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusSupportsCondition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusSupportsCondition;
impl FormatBogusNodeRule<CssBogusSupportsCondition> for FormatCssBogusSupportsCondition {}
