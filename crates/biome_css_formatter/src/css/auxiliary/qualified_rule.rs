use crate::prelude::*;
use crate::utils::block_like::FormatSelectorBlockBoundary;
use biome_css_syntax::{CssQualifiedRule, CssQualifiedRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQualifiedRule;
impl FormatNodeRule<CssQualifiedRule> for FormatCssQualifiedRule {
    fn fmt_fields(&self, node: &CssQualifiedRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQualifiedRuleFields { prelude, block } = node.as_fields();
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
        _: &CssQualifiedRule,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        // Selector/block boundary comments are formatted inside `fmt_fields`.
        Ok(())
    }
}
