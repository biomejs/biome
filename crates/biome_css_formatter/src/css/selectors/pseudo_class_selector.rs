use crate::prelude::*;
use biome_css_syntax::{CssPseudoClassSelector, CssPseudoClassSelectorFields};
use biome_formatter::trivia::format_dangling_comment;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoClassSelector;
impl FormatNodeRule<CssPseudoClassSelector> for FormatCssPseudoClassSelector {
    fn fmt_fields(&self, node: &CssPseudoClassSelector, f: &mut CssFormatter) -> FormatResult<()> {
        let CssPseudoClassSelectorFields { colon_token, class } = node.as_fields();

        write!(f, [colon_token.format()])?;
        let comments = f.comments().clone();
        for comment in comments.dangling_comments(node.syntax()) {
            write!(f, [format_dangling_comment(comment)])?;
        }
        write!(f, [class.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &CssPseudoClassSelector,
        _: &mut CssFormatter,
    ) -> FormatResult<()> {
        Ok(())
    }
}
