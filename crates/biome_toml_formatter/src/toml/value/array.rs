use crate::prelude::*;
use biome_formatter::{Expand, format_args, write};
use biome_toml_syntax::{TomlArray, TomlArrayFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlArray;

impl FormatNodeRule<TomlArray> for FormatTomlArray {
    fn fmt_fields(&self, node: &TomlArray, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlArrayFields {
            l_brack_token,
            elements,
            r_brack_token,
        } = node.as_fields();

        let should_expand = f.comments().has_dangling_comments(node.syntax())
            || match f.options().expand() {
                Expand::Always => !elements.is_empty(),
                Expand::Auto => elements.syntax().has_leading_newline(),
                Expand::Never => false,
            };

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

    fn fmt_dangling_comments(&self, _: &TomlArray, _: &mut TomlFormatter) -> FormatResult<()> {
        // Formatted inside the brackets by `fmt_fields`.
        Ok(())
    }
}
