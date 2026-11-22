use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlLiteralString, TomlLiteralStringFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlLiteralString;

impl FormatNodeRule<TomlLiteralString> for FormatTomlLiteralString {
    fn fmt_fields(&self, node: &TomlLiteralString, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlLiteralStringFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
