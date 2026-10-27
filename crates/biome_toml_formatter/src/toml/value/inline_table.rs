use crate::prelude::*;
use crate::toml::lists::inline_table_entry_list::InlineTableLayout;
use crate::utils::is_multiline_inline_table;
use biome_formatter::{format_args, write};
use biome_toml_syntax::{TomlInlineTable, TomlInlineTableFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlInlineTable;

impl FormatNodeRule<TomlInlineTable> for FormatTomlInlineTable {
    /// Keeps the inline table on one line unless it already spans lines.
    ///
    /// Line breaks inside an inline table are only valid since TOML 1.1, so
    /// breaking a single-line inline table could make the document unreadable
    /// for TOML 1.0 parsers.
    fn fmt_fields(&self, node: &TomlInlineTable, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlInlineTableFields {
            l_curly_token,
            entries,
            r_curly_token,
        } = node.as_fields();

        if is_multiline_inline_table(node) {
            write!(
                f,
                [
                    l_curly_token.format(),
                    block_indent(&format_args![
                        entries.format().with_options(InlineTableLayout::Multiline),
                        format_dangling_comments(node.syntax())
                    ]),
                    r_curly_token.format()
                ]
            )
        } else if entries.is_empty() {
            write!(f, [l_curly_token.format(), r_curly_token.format()])
        } else {
            write!(
                f,
                [
                    l_curly_token.format(),
                    space(),
                    entries.format().with_options(InlineTableLayout::SingleLine),
                    space(),
                    r_curly_token.format()
                ]
            )
        }
    }

    fn fmt_dangling_comments(
        &self,
        _: &TomlInlineTable,
        _: &mut TomlFormatter,
    ) -> FormatResult<()> {
        // Formatted inside the braces by `fmt_fields`. Comments can only
        // appear in a multi-line inline table.
        Ok(())
    }
}
