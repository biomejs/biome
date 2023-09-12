use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::TsIndexedAccessTypeFields;
use biome_js_syntax::{JsSyntaxNode, TsIndexedAccessType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIndexedAccessType;

impl FormatNodeRule<TsIndexedAccessType> for FormatTsIndexedAccessType {
    fn fmt_fields(&self, node: &TsIndexedAccessType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIndexedAccessTypeFields {
            object_type,
            l_brack_token,
            index_type,
            r_brack_token,
        } = node.as_fields();
        write![
            f,
            [
                object_type.format(),
                l_brack_token.format(),
                index_type.format(),
                r_brack_token.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsIndexedAccessType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsIndexedAccessType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
