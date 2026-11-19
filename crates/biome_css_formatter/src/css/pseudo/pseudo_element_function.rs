use crate::prelude::*;
use crate::utils::case::pseudo_identifier_case;
use biome_css_syntax::{CssPseudoElementFunction, CssPseudoElementFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssPseudoElementFunction;
impl FormatNodeRule<CssPseudoElementFunction> for FormatCssPseudoElementFunction {
    fn fmt_fields(
        &self,
        node: &CssPseudoElementFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssPseudoElementFunctionFields {
            name,
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();
        let name = name?;
        let content = format_with(|f| {
            if items.is_empty() {
                format_dangling_comments(node.syntax()).fmt(f)
            } else {
                items.format().fmt(f)
            }
        });

        write!(
            f,
            [
                name.format().with_text_case(pseudo_identifier_case(&name)),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(&content, should_insert_space),
                    r_paren_token.format()
                ])
            ]
        )
    }

    fn fmt_dangling_comments(
        &self,
        node: &CssPseudoElementFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        if node.items().is_empty() {
            Ok(())
        } else {
            format_dangling_comments(node.syntax())
                .with_soft_block_indent()
                .fmt(f)
        }
    }
}
