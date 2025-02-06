use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use biome_formatter::token::number::{format_number_token, NumberFormatOptions};
use biome_formatter::write;
use biome_js_syntax::JsLiteralMemberNameFields;
use biome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLiteralMemberName;

impl FormatNodeRule<JsLiteralMemberName> for FormatJsLiteralMemberName {
    fn fmt_fields(&self, node: &JsLiteralMemberName, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write![
                    f,
                    [FormatLiteralStringToken::new(
                        &value,
                        StringLiteralParentKind::Member
                    )]
                ]
            }
            JsSyntaxKind::JS_NUMBER_LITERAL => format_number_token(
                &value,
                NumberFormatOptions::default().keep_one_trailing_decimal_zero(),
            )
            .fmt(f),
            _ => write![f, [value.format()]],
        }
    }
}
