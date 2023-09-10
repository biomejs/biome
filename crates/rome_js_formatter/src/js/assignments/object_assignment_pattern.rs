use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use crate::utils::JsObjectPatternLike;
use biome_js_syntax::{JsObjectAssignmentPattern, JsSyntaxNode};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectAssignmentPattern;

impl FormatNodeRule<JsObjectAssignmentPattern> for FormatJsObjectAssignmentPattern {
    fn fmt_fields(
        &self,
        node: &JsObjectAssignmentPattern,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        write!(f, [JsObjectPatternLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &JsObjectAssignmentPattern) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsObjectAssignmentPattern,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside of `JsObjectPatternLike`
        Ok(())
    }
}

impl NeedsParentheses for JsObjectAssignmentPattern {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
