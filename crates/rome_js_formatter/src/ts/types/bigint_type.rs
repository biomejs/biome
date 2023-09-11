use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsBigintType, TsBigintTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsBigintType;

impl FormatNodeRule<TsBigintType> for FormatTsBigintType {
    fn fmt_fields(&self, node: &TsBigintType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigintTypeFields { bigint_token } = node.as_fields();

        write![f, [bigint_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsBigintType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsBigintType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
