use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlMultilineLiteralString, TomlMultilineLiteralStringFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlMultilineLiteralString;

impl FormatNodeRule<TomlMultilineLiteralString> for FormatTomlMultilineLiteralString {
    fn fmt_fields(
        &self,
        node: &TomlMultilineLiteralString,
        f: &mut TomlFormatter,
    ) -> FormatResult<()> {
        let TomlMultilineLiteralStringFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
