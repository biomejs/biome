use crate::prelude::*;
use biome_css_syntax::{CssFontFeatureValuesAtRule, CssFontFeatureValuesAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFontFeatureValuesAtRule;

impl FormatNodeRule<CssFontFeatureValuesAtRule> for FormatCssFontFeatureValuesAtRule {
    fn fmt_fields(
        &self,
        node: &CssFontFeatureValuesAtRule,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssFontFeatureValuesAtRuleFields {
            font_feature_values_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                font_feature_values_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
