use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusLayer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusLayer;
impl FormatBogusNodeRule<CssBogusLayer> for FormatCssBogusLayer {}
