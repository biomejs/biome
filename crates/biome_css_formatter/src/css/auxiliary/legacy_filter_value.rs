use crate::prelude::*;
use biome_css_syntax::CssLegacyFilterValue;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssLegacyFilterValue;
impl FormatNodeRule<CssLegacyFilterValue> for FormatCssLegacyFilterValue {
    fn fmt_fields(&self, node: &CssLegacyFilterValue, f: &mut CssFormatter) -> FormatResult<()> {
        node.components().format().fmt(f)
    }
}
