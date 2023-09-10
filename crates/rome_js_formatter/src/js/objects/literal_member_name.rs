use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use biome_js_syntax::JsLiteralMemberNameFields;
use biome_js_syntax::{JsLiteralMemberName, JsSyntaxKind};
use rome_formatter::write;

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
            _ => write![f, [value.format()]],
        }
    }
}
