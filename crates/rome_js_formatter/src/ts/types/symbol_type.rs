use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsSymbolType, TsSymbolTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsSymbolType;

impl FormatNodeRule<TsSymbolType> for FormatTsSymbolType {
    fn fmt_fields(&self, node: &TsSymbolType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsSymbolTypeFields { symbol_token } = node.as_fields();

        write![f, [symbol_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsSymbolType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsSymbolType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
