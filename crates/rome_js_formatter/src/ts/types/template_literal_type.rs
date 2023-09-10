use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::TsTemplateLiteralTypeFields;
use biome_js_syntax::{JsSyntaxNode, TsTemplateLiteralType};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateLiteralType;

impl FormatNodeRule<TsTemplateLiteralType> for FormatTsTemplateLiteralType {
    fn fmt_fields(&self, node: &TsTemplateLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTemplateLiteralTypeFields {
            l_tick_token,
            elements,
            r_tick_token,
        } = node.as_fields();

        write![
            f,
            [
                l_tick_token.format(),
                elements.format(),
                r_tick_token.format(),
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsTemplateLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTemplateLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
