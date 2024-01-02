use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusProperty;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusProperty;
impl FormatBogusNodeRule<CssBogusProperty> for FormatCssBogusProperty {}
