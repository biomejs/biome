use crate::prelude::*;
use crate::utils::FormatTableEntries;
use biome_formatter::write;
use biome_toml_syntax::{TomlArrayTable, TomlArrayTableFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlArrayTable;

impl FormatNodeRule<TomlArrayTable> for FormatTomlArrayTable {
    fn fmt_fields(&self, node: &TomlArrayTable, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlArrayTableFields {
            l_double_brack_token,
            key,
            r_double_brack_token,
            entries,
        } = node.as_fields();

        write!(
            f,
            [
                l_double_brack_token.format(),
                key.format(),
                r_double_brack_token.format(),
                FormatTableEntries::new(node.syntax(), &entries)
            ]
        )
    }

    fn fmt_dangling_comments(&self, _: &TomlArrayTable, _: &mut TomlFormatter) -> FormatResult<()> {
        // Formatted after the header by `FormatTableEntries`.
        Ok(())
    }
}
