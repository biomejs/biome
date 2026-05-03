use crate::prelude::*;
use biome_css_syntax::{
    CssDeclarationOrRuleBlock, ScssAtRootAtRule, ScssAtRootAtRuleFields, ScssAtRootSelector,
};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssAtRootAtRule;

impl FormatNodeRule<ScssAtRootAtRule> for FormatScssAtRootAtRule {
    fn fmt_fields(&self, node: &ScssAtRootAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssAtRootAtRuleFields {
            at_root_token,
            query,
            selector,
            block,
        } = node.as_fields();

        write!(f, [at_root_token.format()])?;

        if let Some(query) = query {
            write!(f, [space(), query.format()])?;
        }

        let should_break_before_block = selector
            .as_ref()
            .is_some_and(|selector| has_trailing_line_comment(selector, f))
            || block
                .as_ref()
                .is_ok_and(|block| has_leading_line_comment(block, f));

        if let Some(selector) = selector {
            write!(f, [space(), group(&selector.format())])?;
        }

        if should_break_before_block {
            write!(f, [hard_line_break(), block.format()])
        } else {
            write!(f, [space(), block.format()])
        }
    }
}

fn has_trailing_line_comment(selector: &ScssAtRootSelector, f: &CssFormatter) -> bool {
    f.comments()
        .trailing_comments(selector.syntax())
        .iter()
        .any(|comment| comment.kind().is_line())
}

fn has_leading_line_comment(block: &CssDeclarationOrRuleBlock, f: &CssFormatter) -> bool {
    f.comments()
        .leading_comments(block.syntax())
        .iter()
        .any(|comment| comment.kind().is_line())
}
