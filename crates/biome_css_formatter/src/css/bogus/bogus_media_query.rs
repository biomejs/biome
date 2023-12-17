use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusMediaQuery;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusMediaQuery;
impl FormatBogusNodeRule<CssBogusMediaQuery> for FormatCssBogusMediaQuery {}
