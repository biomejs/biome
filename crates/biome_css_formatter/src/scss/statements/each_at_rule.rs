use crate::prelude::*;
use biome_css_syntax::{ScssEachAtRule, ScssEachAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachAtRule;

impl FormatNodeRule<ScssEachAtRule> for FormatScssEachAtRule {
    fn fmt_fields(&self, node: &ScssEachAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachAtRuleFields {
            each_token,
            bindings,
            in_token,
            iterable,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                each_token.format(),
                space(),
                bindings.format(),
                space(),
                in_token.format(),
                space(),
                iterable.format(),
                space(),
                block.format()
            ]
        )
    }
}
