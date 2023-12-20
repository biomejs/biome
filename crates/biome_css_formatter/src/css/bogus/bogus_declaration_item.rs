use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusDeclarationItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusDeclarationItem;
impl FormatBogusNodeRule<CssBogusDeclarationItem> for FormatCssBogusDeclarationItem {}
