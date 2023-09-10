use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsArrayType, TsArrayTypeFields};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsArrayType;

impl FormatNodeRule<TsArrayType> for FormatTsArrayType {
    fn fmt_fields(&self, node: &TsArrayType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsArrayTypeFields {
            l_brack_token,
            element_type,
            r_brack_token,
        } = node.as_fields();
        write![
            f,
            [
                element_type.format(),
                l_brack_token.format(),
                r_brack_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsArrayType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsArrayType {
    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
