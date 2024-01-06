use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusUrlModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusUrlModifier;
impl FormatBogusNodeRule<CssBogusUrlModifier> for FormatCssBogusUrlModifier {}
