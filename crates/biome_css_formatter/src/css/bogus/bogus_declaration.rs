use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusDeclaration;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusDeclaration;
impl FormatBogusNodeRule<CssBogusDeclaration> for FormatCssBogusDeclaration {}
