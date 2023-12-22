use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusKeyframesItem;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusKeyframesItem;
impl FormatBogusNodeRule<CssBogusKeyframesItem> for FormatCssBogusKeyframesItem {}
