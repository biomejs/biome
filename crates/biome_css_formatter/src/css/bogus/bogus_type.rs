use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusType;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusType;
impl FormatBogusNodeRule<CssBogusType> for FormatCssBogusType {}
