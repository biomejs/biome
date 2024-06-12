use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusKeyframesName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusKeyframesName;
impl FormatBogusNodeRule<CssBogusKeyframesName> for FormatCssBogusKeyframesName {}
