use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_json_syntax::JsonObjectValue;
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonObjectValue;

impl FormatNodeRule<JsonObjectValue> for FormatJsonObjectValue {
    fn fmt_fields(&self, node: &JsonObjectValue, f: &mut JsonFormatter) -> FormatResult<()> {
        let should_expand = node.json_member_list().syntax().has_leading_newline()
            || f.comments().has_dangling_comments(node.syntax());

        let list = format_with(|f| {
            write!(
                f,
                [group(&soft_space_or_block_indent(&format_args![
                    &node.json_member_list().format(),
                    format_dangling_comments(node.syntax()),
                ]))
                .should_expand(should_expand)]
            )
        });

        if f.comments().has_leading_comments(node.syntax()) {
            write!(
                f,
                [
                    format_leading_comments(node.syntax()),
                    group(&format_args![
                        node.l_curly_token().format(),
                        list,
                        node.r_curly_token().format()
                    ]),
                ]
            )
        } else {
            write!(
                f,
                [
                    node.l_curly_token().format(),
                    list,
                    node.r_curly_token().format()
                ]
            )
        }
    }

    fn fmt_leading_comments(&self, _: &JsonObjectValue, _: &mut JsonFormatter) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }

    fn fmt_dangling_comments(
        &self,
        _: &JsonObjectValue,
        _: &mut JsonFormatter,
    ) -> FormatResult<()> {
        // Formatted as part of `fmt_fields`
        Ok(())
    }
}
