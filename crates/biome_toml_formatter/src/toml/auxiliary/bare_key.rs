use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlBareKey, TomlBareKeyFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBareKey;

impl FormatNodeRule<TomlBareKey> for FormatTomlBareKey {
    fn fmt_fields(&self, node: &TomlBareKey, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlBareKeyFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
