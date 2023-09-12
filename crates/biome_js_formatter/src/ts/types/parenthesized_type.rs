use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::TsParenthesizedTypeFields;
use biome_js_syntax::{JsSyntaxNode, TsParenthesizedType};

#[derive(Debug, Clone, Default)]
pub struct FormatTsParenthesizedType;

impl FormatNodeRule<TsParenthesizedType> for FormatTsParenthesizedType {
    fn fmt_fields(&self, node: &TsParenthesizedType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsParenthesizedTypeFields {
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [l_paren_token.format(), &ty.format(), r_paren_token.format()]
        )
    }

    fn needs_parentheses(&self, item: &TsParenthesizedType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsParenthesizedType {
    #[inline]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
