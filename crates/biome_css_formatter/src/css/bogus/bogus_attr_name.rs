use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusAttrName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusAttrName;
impl FormatBogusNodeRule<CssBogusAttrName> for FormatCssBogusAttrName {}
