use crate::prelude::*;
use crate::utils::JsObjectPatternLike;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::JsObjectAssignmentPattern;

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
