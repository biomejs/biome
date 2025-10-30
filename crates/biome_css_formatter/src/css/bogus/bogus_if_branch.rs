use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusIfBranch;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusIfBranch;
impl FormatBogusNodeRule<CssBogusIfBranch> for FormatCssBogusIfBranch {}
