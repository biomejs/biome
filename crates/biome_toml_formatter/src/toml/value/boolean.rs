use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlBoolean, TomlBooleanFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBoolean;

impl FormatNodeRule<TomlBoolean> for FormatTomlBoolean {
    fn fmt_fields(&self, node: &TomlBoolean, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlBooleanFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
