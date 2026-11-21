use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlString, TomlStringFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlString;

impl FormatNodeRule<TomlString> for FormatTomlString {
    fn fmt_fields(&self, node: &TomlString, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlStringFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
