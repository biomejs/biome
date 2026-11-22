use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlTime, TomlTimeFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlTime;

impl FormatNodeRule<TomlTime> for FormatTomlTime {
    fn fmt_fields(&self, node: &TomlTime, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlTimeFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
