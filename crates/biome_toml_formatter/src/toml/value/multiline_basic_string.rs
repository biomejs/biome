use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlMultilineBasicString, TomlMultilineBasicStringFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlMultilineBasicString;

impl FormatNodeRule<TomlMultilineBasicString> for FormatTomlMultilineBasicString {
    fn fmt_fields(
        &self,
        node: &TomlMultilineBasicString,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        let TomlMultilineBasicStringFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
