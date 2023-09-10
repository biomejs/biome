use crate::parentheses::NeedsParentheses;
use crate::prelude::*;
use crate::utils::JsObjectLike;
use biome_js_syntax::{JsSyntaxNode, TsObjectType};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsObjectType;

impl FormatNodeRule<TsObjectType> for FormatTsObjectType {
    fn fmt_fields(&self, node: &TsObjectType, f: &mut JsFormatter) -> FormatResult<()> {
        write!(f, [JsObjectLike::from(node.clone())])
    }

    fn needs_parentheses(&self, item: &TsObjectType) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(&self, _: &TsObjectType, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted inside of `JsObjectLike`
        Ok(())
    }
}

impl NeedsParentheses for TsObjectType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
