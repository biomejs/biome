use crate::prelude::*;

use biome_js_syntax::JsImportAssertion;
use biome_js_syntax::JsImportAssertionFields;
use rome_formatter::write;

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

        write![
            f,
            [assertion_kind.format(), space(), l_curly_token.format()]
        ]?;

        if assertions.is_empty() {
            let has_dangling = f.comments().has_dangling_comments(node.syntax());
            write!(
                f,
                [
                    has_dangling.then_some(space()),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    has_dangling.then_some(space()),
                ]
            )?;
        } else {
            write!(
                f,
                [group(&soft_space_or_block_indent(&assertions.format()))]
            )?;
        }

        write!(f, [r_curly_token.format()])
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
