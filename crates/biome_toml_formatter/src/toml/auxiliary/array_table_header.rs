use crate::prelude::*;
use biome_formatter::write;
use biome_toml_syntax::{TomlArrayTableHeader, TomlArrayTableHeaderFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlArrayTableHeader;

impl FormatNodeRule<TomlArrayTableHeader> for FormatTomlArrayTableHeader {
    fn fmt_fields(&self, node: &TomlArrayTableHeader, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlArrayTableHeaderFields {
            l_double_brack_token,
            key,
            r_double_brack_token,
        } = node.as_fields();

        write!(
            f,
            [
                l_double_brack_token.format(),
                key.format(),
                r_double_brack_token.format()
            ]
        )
    }
}
