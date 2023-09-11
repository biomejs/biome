use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::JsFunctionBody;
use biome_js_syntax::JsFunctionBodyFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsFunctionBody;

impl FormatNodeRule<JsFunctionBody> for FormatJsFunctionBody {
    fn fmt_fields(&self, node: &JsFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFunctionBodyFields {
            l_curly_token,
            directives,
            statements,
            r_curly_token,
        } = node.as_fields();

        let r_curly_token = r_curly_token?;

        if statements.is_empty() && directives.is_empty() {
            write!(
                f,
                [
                    l_curly_token.format(),
                    format_dangling_comments(node.syntax()).with_block_indent(),
                    r_curly_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    block_indent(&format_args![directives.format(), statements.format()]),
                    r_curly_token.format(),
                ]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &JsFunctionBody, _: &mut JsFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
