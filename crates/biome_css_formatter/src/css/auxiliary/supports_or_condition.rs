use crate::prelude::*;
use biome_css_syntax::{CssSupportsOrCondition, CssSupportsOrConditionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssSupportsOrCondition;
impl FormatNodeRule<CssSupportsOrCondition> for FormatCssSupportsOrCondition {
    fn fmt_fields(&self, node: &CssSupportsOrCondition, f: &mut CssFormatter) -> FormatResult<()> {
        let CssSupportsOrConditionFields {
            left,
            or_token,
            right,
        } = node.as_fields();

        write!(
            f,
            [
                left?.format().with_text_case(CssCase::Preserve),
                space(),
                or_token.format()?.with_text_case(CssCase::Preserve),
                soft_line_break_or_space(),
                right?.format().with_text_case(CssCase::Preserve)
            ]
        )
    }
}
