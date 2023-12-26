use crate::prelude::*;
use biome_css_syntax::CssFontFeatureValuesAtRule;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesAtRule;

impl FormatNodeRule<CssFontFeatureValuesAtRule> for FormatCssFontFeatureValuesAtRule {
    fn fmt_fields(
        &self,
        node: &CssFontFeatureValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
