use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusPageSelectorPseudo;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusPageSelectorPseudo;
impl FormatBogusNodeRule<CssBogusPageSelectorPseudo> for FormatCssBogusPageSelectorPseudo {}
