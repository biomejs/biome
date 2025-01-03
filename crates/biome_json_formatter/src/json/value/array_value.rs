use crate::prelude::*;
use biome_formatter::{format_args, write, FormatContext};
use biome_json_syntax::{JsonArrayValue, JsonArrayValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayValue;
impl FormatNodeRule<JsonArrayValue> for FormatJsonArrayValue {
    fn fmt_fields(&self, node: &JsonArrayValue, f: &mut JsonFormatter) -> FormatResult<()> {
        let JsonArrayValueFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        let should_expand =
            f.comments().has_dangling_comments(node.syntax()) || f.context().options().expand();

        write!(
            f,
            [
                l_brack_token.format(),
                group(&soft_block_indent(&format_args![
                    elements.format(),
                    format_dangling_comments(node.syntax())
                ]))
                .should_expand(should_expand),
                line_suffix_boundary(),
                r_brack_token.format()
            ]
        )
    }

    fn fmt_dangling_comments(&self, _: &JsonArrayValue, _: &mut JsonFormatter) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }
}
