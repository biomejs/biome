use crate::prelude::*;
use biome_css_syntax::{ScssForAtRule, ScssForAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssForAtRule;

impl FormatNodeRule<ScssForAtRule> for FormatScssForAtRule {
    fn fmt_fields(&self, node: &ScssForAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssForAtRuleFields {
            for_token,
            variable,
            from_token,
            lower_bound,
            operator,
            upper_bound,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                for_token.format(),
                space(),
                variable.format(),
                space(),
                from_token.format(),
                space(),
                lower_bound.format(),
                space(),
                operator.format(),
                space(),
                upper_bound.format(),
                space(),
                block.format()
            ]
        )
    }
}
