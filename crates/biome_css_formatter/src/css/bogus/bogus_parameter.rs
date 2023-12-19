use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusParameter;
impl FormatBogusNodeRule<CssBogusParameter> for FormatCssBogusParameter {}
