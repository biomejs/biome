use crate::prelude::*;
use crate::utils::FormatInterpreterToken;
use biome_formatter::write;

use biome_js_syntax::JsScript;
use biome_js_syntax::JsScriptFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsScript;

impl FormatNodeRule<JsScript> for FormatJsScript {
    fn fmt_fields(&self, node: &JsScript, f: &mut JsFormatter) -> FormatResult<()> {
        let JsScriptFields {
            bom_token,
            interpreter_token,
            directives,
            statements,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                bom_token.format(),
                FormatInterpreterToken::new(interpreter_token.as_ref()),
                format_leading_comments(node.syntax()),
                directives.format(),
            ]
        ]?;

        write![
            f,
            [
                statements.format(),
                format_trailing_comments(node.syntax()),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        ]
    }

    fn fmt_leading_comments(&self, _: &JsScript, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, node: &JsScript, f: &mut JsFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(node.syntax()),
            "Scrip should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &JsScript, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
