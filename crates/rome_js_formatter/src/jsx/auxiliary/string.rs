use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use biome_js_syntax::JsxString;
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxString;

impl FormatNodeRule<JsxString> for FormatJsxString {
    fn fmt_fields(&self, node: &JsxString, f: &mut JsFormatter) -> FormatResult<()> {
        write![
            f,
            [FormatLiteralStringToken::new(
                &node.value_token()?,
                StringLiteralParentKind::Expression
            )]
        ]
    }
}
