use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsNullLiteralType, TsNullLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNullLiteralType;

impl FormatNodeRule<TsNullLiteralType> for FormatTsNullLiteralType {
    fn fmt_fields(&self, node: &TsNullLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNullLiteralTypeFields { literal_token } = node.as_fields();
        write![f, [literal_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNullLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNullLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
