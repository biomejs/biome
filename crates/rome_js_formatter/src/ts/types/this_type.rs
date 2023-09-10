use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsThisType, TsThisTypeFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsThisType;

impl FormatNodeRule<TsThisType> for FormatTsThisType {
    fn fmt_fields(&self, node: &TsThisType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsThisTypeFields { this_token } = node.as_fields();

        write![f, [this_token.format()]]
    }

    fn needs_parentheses(&self, item: &TsThisType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsThisType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
