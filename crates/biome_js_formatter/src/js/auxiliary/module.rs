use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatInterpreterToken;

use biome_js_syntax::JsModule;
use biome_js_syntax::JsModuleFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsModule;

impl FormatNodeRule<JsModule> for FormatJsModule {
    fn fmt_fields(&self, node: &JsModule, f: &mut JsFormatter) -> FormatResult<()> {
        let JsModuleFields {
            bom_token,
            interpreter_token,
            directives,
            items,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                bom_token.format(),
                FormatInterpreterToken::new(interpreter_token.as_ref()),
                format_leading_comments(node.syntax()),
                directives.format()
            ]
        ]?;

        write!(
            f,
            [
                items.format(),
                format_trailing_comments(node.syntax()),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        )
    }

    fn fmt_leading_comments(&self, _: &JsModule, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(&self, module: &JsModule, f: &mut JsFormatter) -> FormatResult<()> {
        debug_assert!(
            !f.comments().has_dangling_comments(module.syntax()),
            "Module should never have dangling comments."
        );
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &JsModule, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
