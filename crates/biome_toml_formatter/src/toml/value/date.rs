use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlDate, TomlDateFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlDate;

impl FormatNodeRule<TomlDate> for FormatTomlDate {
    fn fmt_fields(&self, node: &TomlDate, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlDateFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
