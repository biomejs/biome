use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlQuotedKey, TomlQuotedKeyFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlQuotedKey;

impl FormatNodeRule<TomlQuotedKey> for FormatTomlQuotedKey {
    fn fmt_fields(&self, node: &TomlQuotedKey, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlQuotedKeyFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
