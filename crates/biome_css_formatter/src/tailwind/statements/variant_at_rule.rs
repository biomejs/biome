use crate::prelude::*;
use biome_css_syntax::{TwVariantAtRule, TwVariantAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwVariantAtRule;
impl FormatNodeRule<TwVariantAtRule> for FormatTwVariantAtRule {
    fn fmt_fields(&self, node: &TwVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwVariantAtRuleFields {
            variant_token,
            name,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                variant_token.format(),
                space(),
                name.format(),
                space(),
                block.format()
            ]
        )
    }
}
