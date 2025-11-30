use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusIfTest;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusIfTest;
impl FormatBogusNodeRule<CssBogusIfTest> for FormatCssBogusIfTest {}
