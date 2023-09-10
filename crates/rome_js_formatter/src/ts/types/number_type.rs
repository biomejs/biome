use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsNumberType, TsNumberTypeFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNumberType;

impl FormatNodeRule<TsNumberType> for FormatTsNumberType {
    fn fmt_fields(&self, node: &TsNumberType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNumberTypeFields { number_token } = node.as_fields();

        write![f, [number_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNumberType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNumberType {
    fn needs_parentheses_with_parent(&self, _: &JsSyntaxNode) -> bool {
        false
    }
}
