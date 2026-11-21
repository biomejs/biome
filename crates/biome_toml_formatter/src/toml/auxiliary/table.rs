use crate::prelude::*;
use crate::utils::FormatTableEntries;
use biome_formatter::write;
use biome_toml_syntax::{TomlTable, TomlTableFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlTable;

impl FormatNodeRule<TomlTable> for FormatTomlTable {
    fn fmt_fields(&self, node: &TomlTable, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlTableFields { header, entries } = node.as_fields();

        write!(f, [header.format(), FormatTableEntries::new(&entries)])
    }
}
