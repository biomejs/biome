use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsVoidType, TsVoidTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsVoidType;

impl FormatNodeRule<TsVoidType> for FormatTsVoidType {
    fn fmt_fields(&self, node: &TsVoidType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsVoidTypeFields { void_token } = node.as_fields();

        write![f, [void_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsVoidType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsVoidType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
