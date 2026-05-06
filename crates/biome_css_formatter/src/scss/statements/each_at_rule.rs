use crate::prelude::*;
use biome_css_syntax::{ScssEachAtRule, ScssEachAtRuleFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssEachAtRule;

impl FormatNodeRule<ScssEachAtRule> for FormatScssEachAtRule {
    fn fmt_fields(&self, node: &ScssEachAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssEachAtRuleFields {
            each_token,
            header,
            block,
        } = node.as_fields();

        write!(
            f,
            [
                each_token.format(),
                group(&format_args![
                    space(),
                    indent(&group(&header.format())),
                    soft_line_break_or_space()
                ]),
                block.format()
            ]
        )
    }
}
