use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlInteger, TomlIntegerFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlInteger;

impl FormatNodeRule<TomlInteger> for FormatTomlInteger {
    fn fmt_fields(&self, node: &TomlInteger, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlIntegerFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
