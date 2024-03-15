use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::JsCatchDeclaration;
use biome_js_syntax::JsCatchDeclarationFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsCatchDeclaration;

impl FormatNodeRule<JsCatchDeclaration> for FormatJsCatchDeclaration {
    fn fmt_fields(&self, node: &JsCatchDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCatchDeclarationFields {
            l_paren_token,
            binding,
            r_paren_token,
            type_annotation,
        } = node.as_fields();

        let binding = binding?;

        let leading_comment_with_break = f
            .comments()
            .leading_comments(binding.syntax())
            .iter()
            .any(|comment| comment.lines_after() > 0 || comment.kind().is_line());

        let last_parameter_node = type_annotation.as_ref().map_or_else(
            || binding.syntax(),
            |type_annotation| type_annotation.syntax(),
        );

        let trailing_comment_with_break = f
            .comments()
            .trailing_comments(last_parameter_node)
            .iter()
            .any(|comment| comment.lines_before() > 0 || comment.kind().is_line());

        if leading_comment_with_break || trailing_comment_with_break {
            write!(
                f,
                [
                    l_paren_token.format(),
                    soft_block_indent(&format_args![binding.format(), type_annotation.format()]),
                    r_paren_token.format()
                ]
            )
        } else {
            write!(
                f,
                [
                    l_paren_token.format(),
                    binding.format(),
                    type_annotation.format(),
                    r_paren_token.format()
                ]
            )
        }
    }
}
