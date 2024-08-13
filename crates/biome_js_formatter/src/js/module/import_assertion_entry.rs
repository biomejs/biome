use crate::prelude::*;
use biome_formatter::write;

use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use biome_js_syntax::JsImportAssertionEntryFields;
use biome_js_syntax::{JsImportAssertionEntry, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportAssertionEntry;

impl FormatNodeRule<JsImportAssertionEntry> for FormatJsImportAssertionEntry {
    fn fmt_fields(&self, node: &JsImportAssertionEntry, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAssertionEntryFields {
            key,
            colon_token,
            value_token,
        } = node.as_fields();

        let key = key?;

        match key.kind() {
            JsSyntaxKind::JS_STRING_LITERAL => {
                write!(
                    f,
                    [FormatLiteralStringToken::new(
                        &key,
                        StringLiteralParentKind::ImportAttribute
                    ),]
                )?;
            }
            _ => {
                write![f, [key.format()]]?;
            }
        };

        write![f, [colon_token.format(), space()]]?;

        if f.comments().has_dangling_comments(node.syntax()) {
            write!(
                f,
                [space(), format_dangling_comments(node.syntax()), space()]
            )?;
        }

        write!(
            f,
            [FormatLiteralStringToken::new(
                &value_token?,
                StringLiteralParentKind::Expression
            )]
        )
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsImportAssertionEntry,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside `fmt_fields`
        Ok(())
    }
}
