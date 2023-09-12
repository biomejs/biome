use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{JsSyntaxNode, TsStringLiteralType, TsStringLiteralTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsStringLiteralType;

impl FormatNodeRule<TsStringLiteralType> for FormatTsStringLiteralType {
    fn fmt_fields(&self, node: &TsStringLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsStringLiteralTypeFields { literal_token } = node.as_fields();

        write!(
            f,
            [FormatLiteralStringToken::new(
                &literal_token?,
                StringLiteralParentKind::Expression
            )]
        )
    }

    fn needs_parentheses(&self, item: &TsStringLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsStringLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
