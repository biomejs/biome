use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlBasicString, TomlBasicStringFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlBasicString;

impl FormatNodeRule<TomlBasicString> for FormatTomlBasicString {
    fn fmt_fields(&self, node: &TomlBasicString, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlBasicStringFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
