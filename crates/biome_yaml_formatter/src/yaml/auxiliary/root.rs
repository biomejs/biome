use crate::prelude::*;
use biome_formatter::{FormatOptions, write};
use biome_yaml_syntax::{YamlRoot, YamlRootFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatYamlRoot;
impl FormatNodeRule<YamlRoot> for FormatYamlRoot {
    fn fmt_fields(&self, node: &YamlRoot, f: &mut YamlFormatter) -> FormatResult<()> {
        let YamlRootFields {
            documents,
            eof_token: _,
        } = node.as_fields();

        write!(f, [documents.format()])?;

        if f.options().trailing_newline().value() {
            write!(f, [hard_line_break()])?;
        }

        Ok(())
    }
}
