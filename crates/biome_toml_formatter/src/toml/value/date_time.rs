use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlDateTime, TomlDateTimeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlDateTime;

impl FormatNodeRule<TomlDateTime> for FormatTomlDateTime {
    fn fmt_fields(&self, node: &TomlDateTime, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlDateTimeFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
