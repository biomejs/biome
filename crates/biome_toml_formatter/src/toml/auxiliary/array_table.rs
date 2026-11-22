use crate::prelude::*;
use crate::utils::FormatTableMembers;
use biome_formatter::write;
use biome_toml_syntax::{TomlArrayTable, TomlArrayTableFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlArrayTable;

impl FormatNodeRule<TomlArrayTable> for FormatTomlArrayTable {
    fn fmt_fields(&self, node: &TomlArrayTable, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlArrayTableFields { header, members } = node.as_fields();

        write!(f, [header.format(), FormatTableMembers::new(&members)])
    }
}
