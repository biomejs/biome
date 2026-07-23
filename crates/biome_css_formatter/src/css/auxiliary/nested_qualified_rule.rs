use crate::prelude::*;
use crate::utils::block_like::FormatSelectorBlockBoundary;
use biome_css_syntax::{CssNestedQualifiedRule, CssNestedQualifiedRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssNestedQualifiedRule;
impl FormatNodeRule<CssNestedQualifiedRule> for FormatCssNestedQualifiedRule {
    fn fmt_fields(&self, node: &CssNestedQualifiedRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssNestedQualifiedRuleFields { prelude, block } = node.as_fields();
        let block = block?;

        write!(
            f,
            [
                // The selector list gets expanded so that every selector
                // appears on its own line, no matter how long they are.
                group(&prelude.format()).should_expand(true),
                FormatSelectorBlockBoundary::new(node.syntax(), &block),
                block.format()
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssNestedQualifiedRule,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Selector/block boundary comments are formatted inside `fmt_fields`.
        Ok(())
    }
}
