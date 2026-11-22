//! Formatting shared by several node rules.

use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlInlineTable, TomlMemberList};

/// Formats the members on the lines that follow a table header.
///
/// A blank line between the header and its first member is preserved.
pub(crate) struct FormatTableMembers<'a> {
    members: &'a TomlMemberList,
}

impl<'a> FormatTableMembers<'a> {
    pub(crate) fn new(members: &'a TomlMemberList) -> Self {
        Self { members }
    }
}

impl Format<TomlFormatContext> for FormatTableMembers<'_> {
    fn fmt(&self, f: &mut TomlFormatter) -> FormatResult<()> {
        if let Some(first) = self.members.iter().next() {
            if get_lines_before(first.syntax()) > 1 {
                write!(f, [empty_line()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }
        }

        write!(f, [self.members.format()])
    }
}

/// Returns `true` if the inline table has a line break between its braces,
/// outside of its values. Inline tables can only span lines since TOML 1.1.
pub(crate) fn is_multiline_inline_table(table: &TomlInlineTable) -> bool {
    let members = table.members();
    members
        .iter()
        .any(|member| member.is_ok_and(|member| member.syntax().has_leading_newline()))
        || members.elements().any(|element| {
            element.trailing_separator().is_ok_and(|separator| {
                separator.is_some_and(|separator| separator.has_leading_newline())
            })
        })
        || table
            .r_curly_token()
            .is_ok_and(|token| token.has_leading_newline())
        || (members.is_empty() && table.syntax().has_comments_descendants())
}
