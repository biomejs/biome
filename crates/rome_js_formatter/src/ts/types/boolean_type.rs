use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsBooleanType, TsBooleanTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBooleanType;

impl FormatNodeRule<TsBooleanType> for FormatTsBooleanType {
    fn fmt_fields(&self, node: &TsBooleanType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBooleanTypeFields { boolean_token } = node.as_fields();

        write![f, [boolean_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsBooleanType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsBooleanType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
