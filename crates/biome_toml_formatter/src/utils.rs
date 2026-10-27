//! Formatting shared by several node rules.

use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlInlineTable, TomlKeyValueList, TomlSyntaxNode};

/// Formats what follows a table header: the comments at the end of the header
/// line and the key/value pairs on the following lines.
///
/// A blank line between the header and its first entry is preserved.
pub(crate) struct FormatTableEntries<'a> {
    table: &'a TomlSyntaxNode,
    entries: &'a TomlKeyValueList,
}

impl<'a> FormatTableEntries<'a> {
    pub(crate) fn new(table: &'a TomlSyntaxNode, entries: &'a TomlKeyValueList) -> Self {
        Self { table, entries }
    }
}

impl Format<TomlFormatContext> for FormatTableEntries<'_> {
    fn fmt(&self, f: &mut TomlFormatter) -> FormatResult<()> {
        if f.comments().has_dangling_comments(self.table) {
            write!(f, [space(), format_dangling_comments(self.table)])?;
        }

        if let Some(first) = self.entries.iter().next() {
            if get_lines_before(first.syntax()) > 1 {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
        }

        write!(f, [self.entries.format()])
    }
}

/// Returns `true` if the inline table has a line break between its braces,
/// outside of its values. Inline tables can only span lines since TOML 1.1.
pub(crate) fn is_multiline_inline_table(table: &TomlInlineTable) -> bool {
    let entries = table.entries();
    entries.syntax().has_leading_newline()
        || entries.elements().any(|element| {
            element.trailing_separator().is_ok_and(|separator| {
                separator.is_some_and(|separator| separator.has_leading_newline())
            })
        })
        || table
            .r_curly_token()
            .is_ok_and(|token| token.has_leading_newline())
        || (entries.is_empty() && table.syntax().has_comments_descendants())
}
