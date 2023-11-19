use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::JsNamedImportSpecifiers;
use biome_js_syntax::JsNamedImportSpecifiersFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNamedImportSpecifiers;

impl FormatNodeRule<JsNamedImportSpecifiers> for FormatJsNamedImportSpecifiers {
    fn fmt_fields(&self, node: &JsNamedImportSpecifiers, f: &mut JsFormatter) -> FormatResult<()> {
        let JsNamedImportSpecifiersFields {
            l_curly_token,
            specifiers,
            r_curly_token,
        } = node.as_fields();

        write!(f, [l_curly_token.format()])?;

        if specifiers.is_empty() {
            write!(
                f,
                [format_dangling_comments(node.syntax()).with_soft_block_indent()]
            )?;
        } else {
            let should_insert_space_around_brackets = f.options().bracket_spacing().value();
            write!(
                f,
                [group(&soft_block_indent_with_maybe_space(
                    &specifiers.format(),
                    should_insert_space_around_brackets
                ))]
            )?;
        }

        write!(f, [r_curly_token.format()])
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsNamedImportSpecifiers,
        _: &mut JsFormatter,
    ) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}
