use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlTableHeader, TomlTableHeaderFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlTableHeader;

impl FormatNodeRule<TomlTableHeader> for FormatTomlTableHeader {
    fn fmt_fields(&self, node: &TomlTableHeader, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlTableHeaderFields {
            l_brack_token,
            key,
            r_brack_token,
        } = node.as_fields();

        write!(
            f,
            [l_brack_token.format(), key.format(), r_brack_token.format()]
        )
    }
}
