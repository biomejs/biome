use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlKeyValue, TomlKeyValueFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlKeyValue;

impl FormatNodeRule<TomlKeyValue> for FormatTomlKeyValue {
    fn fmt_fields(&self, node: &TomlKeyValue, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlKeyValueFields {
            key,
            eq_token,
            value,
        } = node.as_fields();

        write!(
            f,
            [
                key.format(),
                space(),
                eq_token.format(),
                space(),
                value.format()
            ]
        )
    }
}
