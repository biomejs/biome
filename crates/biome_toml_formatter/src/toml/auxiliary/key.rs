use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlKey, TomlKeyFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlKey;

impl FormatNodeRule<TomlKey> for FormatTomlKey {
    fn fmt_fields(&self, node: &TomlKey, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlKeyFields { parts } = node.as_fields();
        write!(f, [parts.format()])
    }
}
