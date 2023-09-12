use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsNonPrimitiveType, TsNonPrimitiveTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsNonPrimitiveType;

impl FormatNodeRule<TsNonPrimitiveType> for FormatTsNonPrimitiveType {
    fn fmt_fields(&self, node: &TsNonPrimitiveType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNonPrimitiveTypeFields { object_token } = node.as_fields();

        write![f, [object_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNonPrimitiveType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNonPrimitiveType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
