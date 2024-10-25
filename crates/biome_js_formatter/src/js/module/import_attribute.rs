use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsImportAttribute;
use biome_js_syntax::JsImportAttributeFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsImportAttribute;

impl FormatNodeRule<JsImportAttribute> for FormatJsImportAttribute {
    fn fmt_fields(&self, node: &JsImportAttribute, f: &mut JsFormatter) -> FormatResult<()> {
        let JsImportAttributeFields {
            l_curly_token,
            attributes,
            r_curly_token,
            with_token,
        } = node.as_fields();
        if attributes.is_empty() {
            let has_dangling = f.comments().has_dangling_comments(node.syntax());
            write!(
                f,
                [
                    format_removed(&with_token?),
                    format_removed(&l_curly_token?),
                    has_dangling.then_some(space()),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    format_removed(&r_curly_token?),
                ]
            )
        } else {
            let should_insert_space_around_brackets = f.options().bracket_spacing().value();
            write!(
                f,
                [
                    space(),
                    with_token.format(),
                    space(),
                    l_curly_token.format(),
                    group(&soft_block_indent_with_maybe_space(
                        &attributes.format(),
                        should_insert_space_around_brackets
                    )),
                    r_curly_token.format()
                ]
            )
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsImportAttribute,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
