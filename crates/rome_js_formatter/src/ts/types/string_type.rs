use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsStringType, TsStringTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsStringType;

impl FormatNodeRule<TsStringType> for FormatTsStringType {
    fn fmt_fields(&self, node: &TsStringType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsStringTypeFields { string_token } = node.as_fields();

        write![f, [string_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsStringType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsStringType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
