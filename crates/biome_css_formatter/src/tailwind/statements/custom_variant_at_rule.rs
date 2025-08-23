use crate::prelude::*;
use biome_css_syntax::{TwCustomVariantAtRule, TwCustomVariantAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwCustomVariantAtRule;
impl FormatNodeRule<TwCustomVariantAtRule> for FormatTwCustomVariantAtRule {
    fn fmt_fields(&self, node: &TwCustomVariantAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let TwCustomVariantAtRuleFields {
            custom_variant_token,
            name,
            selector,
        } = node.as_fields();

        write!(
            f,
            [
                custom_variant_token.format(),
                space(),
                name.format(),
                space(),
                selector.format()
            ]
        )
    }
}
