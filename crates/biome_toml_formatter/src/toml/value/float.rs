use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlFloat, TomlFloatFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlFloat;

impl FormatNodeRule<TomlFloat> for FormatTomlFloat {
    fn fmt_fields(&self, node: &TomlFloat, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlFloatFields { value_token } = node.as_fields();
        write!(f, [value_token.format()])
    }
}
