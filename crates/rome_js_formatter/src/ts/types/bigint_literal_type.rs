use std::borrow::Cow;

use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_js_syntax::{JsSyntaxNode, TsBigintLiteralType, TsBigintLiteralTypeFields};
use rome_formatter::token::string::ToAsciiLowercaseCow;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsBigintLiteralType;

impl FormatNodeRule<TsBigintLiteralType> for FormatTsBigintLiteralType {
    fn fmt_fields(&self, node: &TsBigintLiteralType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsBigintLiteralTypeFields {
            minus_token,
            literal_token,
        } = node.as_fields();
        write![f, [minus_token.format()]]?;
        let literal_token = literal_token?;

        let original = literal_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write![f, [literal_token.format()]],
            Cow::Owned(lowercase) => {
                write!(
                    f,
                    [format_replaced(
                        &literal_token,
                        &dynamic_text(&lowercase, literal_token.text_trimmed_range().start())
                    )]
                )
            }
        }
    }

    fn needs_parentheses(&self, item: &TsBigintLiteralType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsBigintLiteralType {
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}
