use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusBody;
impl FormatBogusNodeRule<CssBogusBody> for FormatCssBogusBody {}
