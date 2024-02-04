use crate::prelude::*;
use biome_css_syntax::{CssNestedQualifiedRule, CssNestedQualifiedRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNestedQualifiedRule;
impl FormatNodeRule<CssNestedQualifiedRule> for FormatCssNestedQualifiedRule {
    fn fmt_fields(&self, node: &CssNestedQualifiedRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNestedQualifiedRuleFields { prelude, block } = node.as_fields();

        write!(
            f,
            [
                // The selector list gets expanded so that every selector
                // appears on its own line, no matter how long they are.
                group(&prelude.format()).should_expand(true),
                space(),
                &block?.format()
            ]
        )
    }
}
