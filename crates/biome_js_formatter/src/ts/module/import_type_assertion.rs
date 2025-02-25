use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::TsImportTypeAssertion;
use biome_js_syntax::TsImportTypeAssertionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsImportTypeAssertion;
impl FormatNodeRule<TsImportTypeAssertion> for FormatTsImportTypeAssertion {
    fn fmt_fields(&self, node: &TsImportTypeAssertion, f: &mut JsFormatter) -> FormatResult<()> {
        let TsImportTypeAssertionFields {
            colon_token,
            l_curly_token,
            assertions,
            r_curly_token,
            with_token,
        } = node.as_fields();

        if assertions.is_empty() {
            let has_dangling = f.comments().has_dangling_comments(node.syntax());
            write!(
                f,
                [
                    format_removed(&with_token?),
                    format_removed(&colon_token?),
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
                    colon_token.format(),
                    space(),
                    l_curly_token.format(),
                    group(&soft_block_indent_with_maybe_space(
                        &assertions.format(),
                        should_insert_space_around_brackets
                    )),
                    r_curly_token.format()
                ]
            )
        }
    }
}
