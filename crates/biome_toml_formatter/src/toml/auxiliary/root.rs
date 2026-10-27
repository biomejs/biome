use crate::prelude::*;
use biome_formatter::{FormatOptions, write};
use biome_toml_syntax::{TomlRoot, TomlRootFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTomlRoot;

impl FormatNodeRule<TomlRoot> for FormatTomlRoot {
    fn fmt_fields(&self, node: &TomlRoot, f: &mut TomlFormatter) -> FormatResult<()> {
        let TomlRootFields {
            bom_token,
            items,
            eof_token,
        } = node.as_fields();

        write!(
            f,
            [
                bom_token.format(),
                items.format(),
                format_removed(&eof_token?)
            ]
        )?;

        if f.options().trailing_newline().value() {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}
