use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsNeverType, TsNeverTypeFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsNeverType;

impl FormatNodeRule<TsNeverType> for FormatTsNeverType {
    fn fmt_fields(&self, node: &TsNeverType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsNeverTypeFields { never_token } = node.as_fields();
        write![f, [never_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsNeverType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsNeverType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
