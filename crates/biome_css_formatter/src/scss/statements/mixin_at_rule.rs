use crate::prelude::*;
use biome_css_syntax::{ScssMixinAtRule, ScssMixinAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssMixinAtRule;

impl FormatNodeRule<ScssMixinAtRule> for FormatScssMixinAtRule {
    fn fmt_fields(&self, node: &ScssMixinAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssMixinAtRuleFields {
            mixin_token,
            name,
            parameters,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                mixin_token.format()?.with_text_case(CssCase::Lowercase),
                space(),
                name?.format().with_text_case(CssCase::Preserve),
                parameters.format(),
                space(),
                block.format()
            ]
        )
    }
}
