use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusComponentValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusComponentValue;
impl FormatBogusNodeRule<CssBogusComponentValue> for FormatCssBogusComponentValue {}
