use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsUndefinedType, TsUndefinedTypeFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsUndefinedType;

impl FormatNodeRule<TsUndefinedType> for FormatTsUndefinedType {
    fn fmt_fields(&self, node: &TsUndefinedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUndefinedTypeFields { undefined_token } = node.as_fields();

        write![f, [undefined_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsUndefinedType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsUndefinedType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
