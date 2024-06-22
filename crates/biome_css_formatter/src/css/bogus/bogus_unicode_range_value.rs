use crate::FormatBogusNodeRule;
use biome_css_syntax::CssBogusUnicodeRangeValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssBogusUnicodeRangeValue;
impl FormatBogusNodeRule<CssBogusUnicodeRangeValue> for FormatCssBogusUnicodeRangeValue {}
