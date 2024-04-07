use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusCustomIdentifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusCustomIdentifier;
impl FormatBogusNodeRule<CssBogusCustomIdentifier> for FormatCssBogusCustomIdentifier {}
