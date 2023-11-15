use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportAssertion;
use biome_js_syntax::JsImportAssertionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportAssertion;

impl FormatNodeRule<JsImportAssertion> for FormatJsImportAssertion {
    fn fmt_fields(&self, node: &JsImportAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAssertionFields {
            l_curly_token,
            assertions,
            r_curly_token,
            assertion_kind,
        } = node.as_fields();
        if assertions.is_empty() {
            let has_dangling = f.comments().has_dangling_comments(node.syntax());
            write!(
                f,
                [
                    format_removed(&assertion_kind?),
                    format_removed(&l_curly_token?),
                    has_dangling.then_some(space()),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    format_removed(&r_curly_token?),
                ]
            )
        } else {
            write!(
                f,
                [
                    space(),
                    assertion_kind.format(),
                    space(),
                    l_curly_token.format(),
                    group(&soft_space_or_block_indent(&assertions.format())),
                    r_curly_token.format()
                ]
            )
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsImportAssertion,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
