use crate::prelude::*;
use crate::utils::FormatTableEntries;
use biome_formatter::write;
use biome_toml_syntax::{TomlTable, TomlTableFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlTable;

impl FormatNodeRule<TomlTable> for FormatTomlTable {
    fn fmt_fields(&self, node: &TomlTable, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlTableFields {
            l_brack_token,
            key,
            r_brack_token,
            entries,
        } = node.as_fields();

        write!(
            f,
            [
                l_brack_token.format(),
                key.format(),
                r_brack_token.format(),
                FormatTableEntries::new(node.syntax(), &entries)
            ]
        )
    }

    fn fmt_dangling_comments(&self, _: &TomlTable, _: &mut TomlFormatter) -> FormatResult<()> {
        // Formatted after the header by `FormatTableEntries`.
        Ok(())
    }
}
