use crate::prelude::*;
use biome_css_syntax::{ScssFunctionAtRule, ScssFunctionAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssFunctionAtRule;

impl FormatNodeRule<ScssFunctionAtRule> for FormatScssFunctionAtRule {
    fn fmt_fields(&self, node: &ScssFunctionAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssFunctionAtRuleFields {
            function_token,
            name,
            parameters,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                function_token.format(),
                space(),
                name.format(),
                parameters.format(),
                space(),
                block.format()
            ]
        )
    }
}
