use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusDocumentMatcher;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusDocumentMatcher;
impl FormatBogusNodeRule<CssBogusDocumentMatcher> for FormatCssBogusDocumentMatcher {}
