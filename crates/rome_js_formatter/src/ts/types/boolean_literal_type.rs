use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsBooleanLiteralType, TsBooleanLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBooleanLiteralType;

impl FormatNodeRule<TsBooleanLiteralType> for FormatTsBooleanLiteralType {
    fn fmt_fields(&self, node: &TsBooleanLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBooleanLiteralTypeFields { literal } = node.as_fields();
        write![f, [literal.format()]]
    }

    fn needs_parentheses(&self, item: &TsBooleanLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsBooleanLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
