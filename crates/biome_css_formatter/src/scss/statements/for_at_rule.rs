use crate::prelude::*;
use biome_css_syntax::{ScssForAtRule, ScssForAtRuleFields};
use biome_formatter::{format_args, write};

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
                for_token.format()?.with_text_case(CssCase::Lowercase),
                space(),
                group(&format_args![
                    variable.format(),
                    indent(&format_args![
                        soft_line_break_or_space(),
                        from_token.format()?.with_text_case(CssCase::Preserve),
                        space(),
                        lower_bound.format(),
                        soft_line_break_or_space(),
                        operator.format()?.with_text_case(CssCase::Preserve),
                        space(),
                        upper_bound.format()
                    ]),
                    soft_line_break_or_space()
                ]),
                block.format()
            ]
        )
    }
}
