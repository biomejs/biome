use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsUnknownType, TsUnknownTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnknownType;

impl FormatNodeRule<TsUnknownType> for FormatTsUnknownType {
    fn fmt_fields(&self, node: &TsUnknownType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnknownTypeFields { unknown_token } = node.as_fields();

        write![f, [unknown_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsUnknownType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsUnknownType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
